import { createSlice, createSelector } from '@reduxjs/toolkit';
import { RootState } from '.';

export const counterSlice = createSlice({
  name: 'counter',
  initialState: {
    value: 0,
  },
  reducers: {
    increment: (state) => {
      state.value += 1;
    },
    decrement: (state) => {
      state.value -= 1;
    },
  },
});

export const { increment, decrement } = counterSlice.actions;

export const selectCount = createSelector(
  (state: RootState) => state.counter.value,
  (count) => count
);

export default counterSlice.reducer;
