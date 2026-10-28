// Contrôleur UI : Affichage de l'état de connexion dans l'en-tête
import { getCurrentUser } from '../../services/auth/session.js';
import { logoutUser } from '../../services/auth/logout.js';

export async function initAccountBadge() {
  const user = await getCurrentUser();
  if (!user) return;

  const accountLink = document.querySelector('.header-account a[href="/login/"]');
  if (!accountLink) return;

  const nameSpan = document.createElement('span');
  nameSpan.className = 'tab';
  nameSpan.textContent = user.name;

  const logoutButton = document.createElement('button');
  logoutButton.type = 'button';
  logoutButton.className = 'tab';
  logoutButton.textContent = 'Logout';
  logoutButton.addEventListener('click', logoutUser);

  accountLink.replaceWith(nameSpan, logoutButton);
}
