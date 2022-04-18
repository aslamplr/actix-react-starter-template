import { combineReducers, applyMiddleware, createStore } from 'redux';
import counterReducer from './counter';
import apiStatusReducer from './apiStatus';
import mainSaga from '../sagas';
import createSagaMiddleware from '@redux-saga/core';

const sagaMiddleware = createSagaMiddleware();

const rootReducer = combineReducers({
  counter: counterReducer,
  api: apiStatusReducer,
});

const store = createStore(rootReducer, applyMiddleware(sagaMiddleware));

sagaMiddleware.run(mainSaga);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
