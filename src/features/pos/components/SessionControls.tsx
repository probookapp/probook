import { useTranslation } from "react-i18next";
import { LogOut, History, DollarSign, Briefcase } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { usePosStore } from "../stores/usePosStore";

interface SessionControlsProps {
  onCloseSession: () => void;
}

export function SessionControls({ onCloseSession }: SessionControlsProps) {
  const { t } = useTranslation("pos");
  const navigate = useNavigate();
  const { clearCart, items } = usePosStore();
  const [showMenu, setShowMenu] = useState(false);

  return (
    <div className="relative">
      <div className="flex items-center gap-2">
        {items.length > 0 && (
          <button
            onClick={clearCart}
            className="px-3 py-1.5 text-sm bg-white/20 hover:bg-white/30 rounded"
          >
            {t("clearCart")}
          </button>
        )}
        <button
          onClick={() => setShowMenu(!showMenu)}
          className="px-3 py-1.5 text-sm bg-white/20 hover:bg-white/30 rounded flex items-center gap-1"
        >
          {t("menu")}
        </button>
      </div>

      {showMenu && (
        <>
          <div
            className="fixed inset-0 z-10"
            onClick={() => setShowMenu(false)}
          />
          <div className="absolute right-0 top-full mt-2 bg-background text-foreground rounded-lg shadow-xl border z-20 min-w-48">
            <button
              onClick={() => {
                setShowMenu(false);
                // TODO: Show transaction history
              }}
              className="w-full px-4 py-3 flex items-center gap-3 hover:bg-muted text-left"
            >
              <History className="h-4 w-4" />
              {t("transactionHistory")}
            </button>
            <button
              onClick={() => {
                setShowMenu(false);
                // TODO: Show cash movement modal
              }}
              className="w-full px-4 py-3 flex items-center gap-3 hover:bg-muted text-left"
            >
              <DollarSign className="h-4 w-4" />
              {t("cashMovement")}
            </button>
            <hr />
            <button
              onClick={() => {
                setShowMenu(false);
                navigate("/");
              }}
              className="w-full px-4 py-3 flex items-center gap-3 hover:bg-muted text-left"
            >
              <Briefcase className="h-4 w-4" />
              {t("backToOffice")}
            </button>
            <button
              onClick={() => {
                setShowMenu(false);
                onCloseSession();
              }}
              className="w-full px-4 py-3 flex items-center gap-3 hover:bg-muted text-left text-destructive"
            >
              <LogOut className="h-4 w-4" />
              {t("closeSession")}
            </button>
          </div>
        </>
      )}
    </div>
  );
}
