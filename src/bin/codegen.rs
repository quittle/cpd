use cpd::web_actor::BattleState;
use schemars::SchemaGenerator;
use schemars::r#gen::SchemaSettings;
use schemars::schema::Schema;
use std::env;
use std::io::{Error, Write};
use std::process::{Command, Stdio};

// Fix generated typescript turns out to be invalid
// 1. Enums are both declared as a type and extracted from the schemas
fn fix_typescript(typescript_bytes: &[u8]) -> Vec<u8> {
    let enum_declaration_regex = regex::Regex::new(r"export enum (\w+)").unwrap();

    let mut typescript = String::from_utf8_lossy(typescript_bytes).to_string();

    let enums: Vec<String> = enum_declaration_regex
        .captures_iter(&typescript)
        .map(|c| c[1].into())
        .collect();

    for enum_name in enums {
        typescript = typescript.replace(
            &format!(
                "export type {0} = components['schemas']['{0}'];\n",
                enum_name
            ),
            "",
        );
    }

    typescript.into_bytes()
}

fn main() -> Result<(), Error> {
    let mut schema =
        SchemaGenerator::new(SchemaSettings::openapi3()).root_schema_for::<BattleState>();
    let mut schemas = schema.definitions;
    schemas.insert(
        schema.schema.metadata().title.take().unwrap(),
        Schema::Object(schema.schema),
    );
    let battle_json = serde_json::json!({
        "openapi": "3.0.0",
        "components": {
            "schemas": schemas
        }
    });
    let json_schema = serde_json::to_string_pretty(&battle_json)?;

    let out_file = env::current_dir()?
        .join("src")
        .join("web_actor")
        .join("static")
        .join("battle-schema.generated.ts");

    let mut child = Command::new("npx")
        .args([
            "openapi-typescript",
            "--enum",
            "--root-types",
            "--root-types-no-schema-prefix",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write json schema to stdin
    child
        .stdin
        .take()
        .unwrap()
        .write_all(json_schema.as_bytes())?;

    let output = child.wait_with_output()?;
    assert!(output.status.success());

    let typescript = fix_typescript(&output.stdout);

    {
        let mut file = std::fs::File::create(&out_file)?;

        // Write the generated header comment to the file
        file.write_fmt(format_args!("// @{}\n\n", "generated"))?;

        // Write the Typescript to the files
        file.write_all(&typescript)?;
    }

    eprintln!("Generated battle schema at {}", out_file.to_string_lossy());

    Ok(())
}
