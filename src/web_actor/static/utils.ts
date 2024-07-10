import { ActionTarget, Card, CardAction } from "./battle";

export function getActionTarget(action: CardAction): ActionTarget {
    return action["Damage"]?.target ?? action["Heal"]?.target;
}

export function getCardTarget(card: Card): ActionTarget {
    let defaultTarget = ActionTarget.Me;
    for (const action of card.actions) {
        const target = getActionTarget(action);
        switch (target) {
            case ActionTarget.Any:
                // Any is any
                return ActionTarget.Any;
            case ActionTarget.Others:
                // If others found, then it should target others
                defaultTarget = ActionTarget.Others;
                break;
            case ActionTarget.Me:
                break;
        }
    }

    return defaultTarget;
}