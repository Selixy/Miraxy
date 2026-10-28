// Service Auth : Déconnexion de l'utilisateur
import { httpClient } from '../../core/http/client.js';
import { ENDPOINTS } from '../../core/http/endpoints.js';

export async function logoutUser() {
  await httpClient(ENDPOINTS.AUTH_LOGOUT, { method: 'POST' });
  location.href = '/';
}
