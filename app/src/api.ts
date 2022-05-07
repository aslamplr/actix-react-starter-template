const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || '';
const API_BASE_PATH = import.meta.env.VITE_API_BASE_PATH;

export const apiFetch = (urlPath: string) =>
  fetch(
    `${API_BASE_URL}${API_BASE_PATH || ''}${
      urlPath.charAt(0) === '/' ? urlPath : '/' + urlPath
    }`
  );

export const fetchApiStatus = async (): Promise<string> => {
  const reps = await apiFetch('/health');
  const { msg } = await reps.json();
  return msg;
};
