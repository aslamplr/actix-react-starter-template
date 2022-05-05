const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || '';

export const apiFetch = (urlPath: string) =>
  fetch(
    `${API_BASE_URL}${urlPath.charAt(0) === '/' ? urlPath : '/' + urlPath}`
  );

export const fetchApiStatus = async (): Promise<string> => {
  const reps = await apiFetch('/api/status');
  const { msg } = await reps.json();
  return msg;
};
