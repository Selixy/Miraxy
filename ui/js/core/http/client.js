// Client HTTP sécurisé : renvoie toujours { ok, status, data }, jamais d'exception non gérée.
export async function httpClient(path, { method = 'GET', body } = {}) {
  const options = {
    method,
    credentials: 'same-origin',
    headers: {},
  };

  if (body !== undefined) {
    options.headers['Content-Type'] = 'application/json';
    options.body = JSON.stringify(body);
  }

  try {
    const response = await fetch(path, options);
    const data = await response.json().catch(() => null);
    return { ok: response.ok, status: response.status, data };
  } catch {
    return {
      ok: false,
      status: 0,
      data: { message: 'Le serveur est injoignable.' },
    };
  }
}
