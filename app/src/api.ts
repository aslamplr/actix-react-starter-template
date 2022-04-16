export const fetchApiStatus = async (): Promise<string> => {
  const reps = await fetch("/api/status");
  const { text } = await reps.json();
  return text;
};
