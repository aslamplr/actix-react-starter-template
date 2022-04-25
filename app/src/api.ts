export const fetchApiStatus = async (): Promise<string> => {
  const reps = await fetch('/api/status');
  const { msg } = await reps.json();
  return msg;
};
