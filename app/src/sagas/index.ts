import { all, call, fork, put, takeLatest } from 'redux-saga/effects';
import * as api from '../api';
import {
  FETCH_API_STATUS,
  fetchApiStatus as fetchApiStatusAction,
  getStatusSuccess as getStatusSuccessAction,
  getStatusError as getStatusErrorAction,
} from '../store/apiStatus';
import { increment as incrementAction } from '../store/counter';

function* fetchApiStatus() {
  try {
    const status: string = yield call(api.fetchApiStatus);
    yield put(getStatusSuccessAction(status));
  } catch (err) {
    yield put(getStatusErrorAction());
  }
}

function* allTakes() {
  yield all([takeLatest(FETCH_API_STATUS, fetchApiStatus)]);
}

function* mainSaga() {
  yield fork(allTakes);
  yield put(fetchApiStatusAction());
  yield put(incrementAction());
}

export default mainSaga;
