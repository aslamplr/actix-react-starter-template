import { createSlice, createSelector } from "@reduxjs/toolkit";
import { RootState } from ".";

const apiStatusSlice = createSlice({
  name: "apiStatus",
  initialState: {
    status: "loading...",
  },
  reducers: {
    getStatusSuccess: (state, action) => {
      state.status = action.payload;
    },
    getStatusError: (state) => {
      state.status = "error!";
    },
  },
});

export const { getStatusError, getStatusSuccess } = apiStatusSlice.actions;

export const FETCH_API_STATUS = "FETCH_API_STATUS";
export const fetchApiStatus = () => ({ type: FETCH_API_STATUS });

export const selectApiStatus = createSelector(
  (state: RootState) => state.api.status,
  (status) => status.toLowerCase()
);

export default apiStatusSlice.reducer;
