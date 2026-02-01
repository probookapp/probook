import { Navigate } from 'react-router-dom';
import { useAuthStore } from '@/stores/useAuthStore';
import type { PermissionKey } from '@/types';

const routePermissions: { path: string; permission: PermissionKey }[] = [
  { path: '/', permission: 'dashboard' },
  { path: '/clients', permission: 'clients' },
  { path: '/products', permission: 'products' },
  { path: '/suppliers', permission: 'suppliers' },
  { path: '/quotes', permission: 'quotes' },
  { path: '/invoices', permission: 'invoices' },
  { path: '/delivery-notes', permission: 'delivery_notes' },
  { path: '/phonebook', permission: 'phonebook' },
  { path: '/reports', permission: 'reports' },
  { path: '/expenses', permission: 'expenses' },
  { path: '/settings', permission: 'settings' },
];

interface ProtectedRouteProps {
  permission: PermissionKey;
  children: React.ReactNode;
}

export function ProtectedRoute({ permission, children }: ProtectedRouteProps) {
  const { hasPermission } = useAuthStore();

  if (!hasPermission(permission)) {
    const firstPermitted = routePermissions.find((r) => hasPermission(r.permission));
    if (firstPermitted) {
      return <Navigate to={firstPermitted.path} replace />;
    }
    return null;
  }

  return <>{children}</>;
}
