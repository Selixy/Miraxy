// Contrôleur UI : Gestion des formulaires d'authentification (login / signup)
import { httpClient } from '../../core/http/client.js';

export function initAuthForm(formSelector = 'form[data-endpoint]') {
  const form = document.querySelector(formSelector);
  if (!form) return;

  const errorBox = form.querySelector('.error');
  const submitButton = form.querySelector('button[type="submit"]');

  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    if (errorBox) errorBox.hidden = true;
    if (submitButton) submitButton.disabled = true;

    const body = Object.fromEntries(new FormData(form));
    const result = await httpClient(form.dataset.endpoint, { method: 'POST', body });

    if (result.ok) {
      location.href = '/';
      return;
    }

    if (errorBox) {
      errorBox.textContent = result.data?.message ?? 'Une erreur est survenue.';
      errorBox.hidden = false;
    }
    if (submitButton) submitButton.disabled = false;
  });
}
