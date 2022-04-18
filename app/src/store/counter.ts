import { RootState } from '.';
import { createSelector, createSlice } from './utils';

const initialState = {
  value: 0,
};

const counterSlice = createSlice('counter', initialState, {
  increment: (state) => {
    state.value += 1;
  },
  decrement: (state) => {
    state.value -= 1;
  },
});

export const { increment, decrement } = counterSlice.actions;

export const selectCount = createSelector(
  (state: RootState) => state.counter.value,
  (count) => count
);

export default counterSlice.reducer;
