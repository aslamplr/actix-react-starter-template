import { configureStore } from '@reduxjs/toolkit';
import counterReducer from './counter';
import apiStatusReducer from './apiStatus';
import mainSaga from '../sagas';
import createSagaMiddleware from '@redux-saga/core';

const sagaMiddleware = createSagaMiddleware();

const store = configureStore({
  reducer: {
    counter: counterReducer,
    api: apiStatusReducer,
  },
  middleware: [sagaMiddleware],
});

sagaMiddleware.run(mainSaga);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
