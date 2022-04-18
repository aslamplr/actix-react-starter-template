import { RootState } from '.';
import { createAction, createSelector, createSlice } from './utils';

const initialState = {
  status: 'loading...',
};

export const FETCH_API_STATUS = 'FETCH_API_STATUS';
export const fetchApiStatus = createAction(FETCH_API_STATUS);

const apiStatusSlice = createSlice('apiStatus', initialState, {
  getStatusSuccess: (state, status) => {
    state.status = status;
  },
  getStatusError: (state, error) => {
    state.status = error;
  },
});

export const { getStatusSuccess, getStatusError } = apiStatusSlice.actions;

export const selectApiStatus = createSelector(
  (state: RootState) => state.api.status,
  (status) => status.toLowerCase()
);

export default apiStatusSlice.reducer;
