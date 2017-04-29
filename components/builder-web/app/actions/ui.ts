export const SET_LAYOUT = "SET_LAYOUT";

export function setLayout(layout: string) {
    return {
        type: SET_LAYOUT,
        payload: layout
    };
}
