import { NavLink } from "react-router-dom";
import { useTranslation } from "react-i18next";
import {
  LayoutDashboard,
  Users,
  Package,
  FileText,
  Receipt,
  Truck,
  BookUser,
  BarChart3,
  Settings,
  X,
} from "lucide-react";
import { cn } from "@/lib/utils";

interface SidebarProps {
  onClose?: () => void;
}

export function Sidebar({ onClose }: SidebarProps) {
  const { t } = useTranslation("navigation");

  const navigation = [
    { name: t("dashboard"), href: "/", icon: LayoutDashboard },
    { name: t("clients"), href: "/clients", icon: Users },
    { name: t("products"), href: "/products", icon: Package },
    { name: t("quotes"), href: "/quotes", icon: FileText },
    { name: t("invoices"), href: "/invoices", icon: Receipt },
    { name: t("deliveryNotes"), href: "/delivery-notes", icon: Truck },
    { name: t("phonebook"), href: "/phonebook", icon: BookUser },
    { name: t("reports"), href: "/reports", icon: BarChart3 },
    { name: t("settings"), href: "/settings", icon: Settings },
  ];

  return (
    <aside className="w-56 lg:w-64 h-full bg-gray-900 dark:bg-gray-950 text-white flex flex-col">
      <div className="p-6 flex items-center justify-between">
        <h1 className="text-xl font-bold">Probook</h1>
        {onClose && (
          <button
            onClick={onClose}
            aria-label={t("closeSidebar")}
            className="p-1 rounded-lg text-gray-400 hover:text-white hover:bg-gray-800 lg:hidden"
          >
            <X className="h-5 w-5" />
          </button>
        )}
      </div>
      <nav className="flex-1 px-4 space-y-1 overflow-y-auto">
        {navigation.map((item) => (
          <NavLink
            key={item.href}
            to={item.href}
            onClick={onClose}
            className={({ isActive }) =>
              cn(
                "flex items-center gap-3 px-4 py-3 rounded-lg text-sm font-medium transition-colors",
                isActive
                  ? "bg-primary-600 text-white"
                  : "text-gray-300 hover:bg-gray-800 hover:text-white"
              )
            }
          >
            <item.icon className="h-5 w-5" />
            {item.name}
          </NavLink>
        ))}
      </nav>
      <div className="p-4 border-t border-gray-800">
        <p className="text-xs text-gray-500">Version 1.0.0</p>
      </div>
    </aside>
  );
}
