import { Navigate } from 'react-router-dom';
import { useAuthStore } from '@/stores/useAuthStore';
import type { PermissionKey } from '@/types';

interface ProtectedRouteProps {
  permission: PermissionKey;
  children: React.ReactNode;
}

export function ProtectedRoute({ permission, children }: ProtectedRouteProps) {
  const { hasPermission } = useAuthStore();

  if (!hasPermission(permission)) {
    return <Navigate to="/" replace />;
  }

  return <>{children}</>;
}
