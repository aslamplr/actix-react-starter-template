import produce, { Draft } from 'immer';
export { createSelector } from 'reselect';

/* eslint-disable @typescript-eslint/no-explicit-any */
export const createAction =
  <Payload = any>(type: string) =>
  (...args: Payload[]) => ({
    type,
    args,
  });
/* eslint-enable @typescript-eslint/no-explicit-any */

export const createReducer =
  <Payload, State>(
    callback: (s: Draft<State>, action: Action<Payload>) => void,
    initialState: State
  ) =>
  (state: State = initialState, action: Action<Payload>) =>
    produce(state, (draftState) => callback(draftState, action));

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type ReducerFn<State, Payload = any> = (
  s: Draft<State>,
  ...p: Payload[]
) => void;

export const createSlice = <State>(
  namespace: string,
  initialState: State | (() => State),
  reducers: {
    [actionType: string]: ReducerFn<State>;
  }
) => {
  const initialState_ =
    typeof initialState === 'function'
      ? (initialState as () => State)()
      : initialState;

  const reducersByType: Record<string, ReducerFn<State>> = {};
  const actions: Record<string, ReturnType<typeof createAction>> = {};

  Object.keys(reducers).forEach((action) => {
    const actionName = `${namespace}/${action}`;
    reducersByType[actionName] = reducers[action];
    actions[action] = createAction(actionName);
  });

  const reducer = createReducer((state, action) => {
    const fn = reducersByType[action.type];
    if (fn) {
      fn(state, ...action.args);
    }
  }, initialState_);

  return { reducer, actions };
};
