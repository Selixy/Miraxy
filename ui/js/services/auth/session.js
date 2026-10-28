// Service Auth : Vérification de la session courante (sans manipulation du DOM)
import { httpClient } from '../../core/http/client.js';
import { ENDPOINTS } from '../../core/http/endpoints.js';

export async function getCurrentUser() {
  const result = await httpClient(ENDPOINTS.AUTH_ME);
  if (result.ok && result.data) {
    return result.data;
  }
  return null;
}
