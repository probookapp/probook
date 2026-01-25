import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Plus, Pencil, Trash2, Search, Eye, Users } from "lucide-react";
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Modal,
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
  Input,
} from "@/components/ui";
import { ClientForm } from "./components/ClientForm";
import { ClientContacts } from "./components/ClientContacts";
import {
  useClients,
  useCreateClient,
  useUpdateClient,
  useDeleteClient,
} from "./hooks/useClients";
import type { Client } from "@/types";
import type { ClientFormData } from "./schemas/clientSchema";

export function ClientsPage() {
  const { t } = useTranslation("clients");
  const { t: tCommon } = useTranslation("common");
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedClient, setSelectedClient] = useState<Client | undefined>();
  const [viewingClient, setViewingClient] = useState<Client | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [deleteConfirmId, setDeleteConfirmId] = useState<string | null>(null);

  const { data: clients, isLoading } = useClients();
  const createClient = useCreateClient();
  const updateClient = useUpdateClient();
  const deleteClient = useDeleteClient();

  const filteredClients = clients?.filter(
    (client) =>
      client.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      client.email?.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const handleOpenModal = (client?: Client) => {
    setSelectedClient(client);
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    setSelectedClient(undefined);
    setIsModalOpen(false);
  };

  const handleSubmit = async (data: ClientFormData) => {
    if (selectedClient) {
      await updateClient.mutateAsync({ ...data, id: selectedClient.id });
    } else {
      await createClient.mutateAsync(data);
    }
    handleCloseModal();
  };

  const handleDelete = async (id: string) => {
    await deleteClient.mutateAsync(id);
    setDeleteConfirmId(null);
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl sm:text-2xl font-bold text-gray-900 dark:text-gray-100">{t("title")}</h1>
          <p className="text-sm sm:text-base text-gray-500 dark:text-gray-400">{t("subtitle")}</p>
        </div>
        <Button onClick={() => handleOpenModal()} size="sm" className="self-start sm:self-auto">
          <Plus className="h-4 w-4 mr-2" />
          {t("newClient")}
        </Button>
      </div>

      <Card>
        <CardHeader>
          <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
            <CardTitle>{t("clientList")}</CardTitle>
            <div className="relative w-full sm:w-56 md:w-64 lg:w-72">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <Input
                id="client-search"
                name="client-search"
                placeholder={t("searchPlaceholder")}
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                autoComplete="off"
                className="pl-9"
              />
            </div>
          </div>
        </CardHeader>
        <CardContent className="p-0 overflow-x-auto">
          <Table className="min-w-150">
            <TableHeader>
              <TableRow>
                <TableHead>{t("fields.name")}</TableHead>
                <TableHead>{t("fields.email")}</TableHead>
                <TableHead>{t("fields.phone")}</TableHead>
                <TableHead>{t("fields.city")}</TableHead>
                <TableHead className="w-24">{tCommon("buttons.actions")}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredClients && filteredClients.length > 0 ? (
                filteredClients.map((client) => (
                  <TableRow key={client.id}>
                    <TableCell className="font-medium text-gray-900 dark:text-gray-100">{client.name}</TableCell>
                    <TableCell className="text-gray-600 dark:text-gray-400">{client.email || "-"}</TableCell>
                    <TableCell className="text-gray-600 dark:text-gray-400">{client.phone || "-"}</TableCell>
                    <TableCell className="text-gray-600 dark:text-gray-400">{client.city || "-"}</TableCell>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <button
                          onClick={() => setViewingClient(client)}
                          className="p-1 text-gray-500 hover:text-primary-600 dark:hover:text-primary-400 transition-colors"
                          title={t("viewContacts")}
                          aria-label={t("viewContacts")}
                        >
                          <Eye className="h-4 w-4" />
                        </button>
                        <button
                          onClick={() => handleOpenModal(client)}
                          className="p-1 text-gray-500 hover:text-primary-600 dark:hover:text-primary-400 transition-colors"
                          title={tCommon("buttons.edit")}
                          aria-label={tCommon("buttons.edit")}
                        >
                          <Pencil className="h-4 w-4" />
                        </button>
                        <button
                          onClick={() => setDeleteConfirmId(client.id)}
                          className="p-1 text-gray-500 hover:text-red-600 transition-colors"
                          title={tCommon("buttons.delete")}
                          aria-label={tCommon("buttons.delete")}
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))
              ) : (
                <TableRow>
                  <TableCell colSpan={5} className="text-center text-gray-500 dark:text-gray-400 py-8">
                    {t("noClients")}
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      <Modal
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        title={selectedClient ? t("editClient") : t("newClient")}
        size="lg"
      >
        <ClientForm
          client={selectedClient}
          onSubmit={handleSubmit}
          onCancel={handleCloseModal}
          isLoading={createClient.isPending || updateClient.isPending}
        />
      </Modal>

      <Modal
        isOpen={!!deleteConfirmId}
        onClose={() => setDeleteConfirmId(null)}
        title={tCommon("messages.confirmDelete")}
        size="sm"
      >
        <p className="text-gray-600 dark:text-gray-400 mb-6">
          {t("deleteConfirmation")}
        </p>
        <div className="flex justify-end gap-3">
          <Button variant="secondary" onClick={() => setDeleteConfirmId(null)}>
            {tCommon("buttons.cancel")}
          </Button>
          <Button
            variant="danger"
            onClick={() => deleteConfirmId && handleDelete(deleteConfirmId)}
            isLoading={deleteClient.isPending}
          >
            {tCommon("buttons.delete")}
          </Button>
        </div>
      </Modal>

      {/* Client Details & Contacts Modal */}
      <Modal
        isOpen={!!viewingClient}
        onClose={() => setViewingClient(null)}
        title={t("clientDetails")}
        size="lg"
      >
        {viewingClient && (
          <div className="space-y-6">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">{t("fields.name")}</p>
                <p className="font-medium text-gray-900 dark:text-gray-100">{viewingClient.name}</p>
              </div>
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">{t("fields.email")}</p>
                <p className="font-medium text-gray-900 dark:text-gray-100">{viewingClient.email || "-"}</p>
              </div>
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">{t("fields.phone")}</p>
                <p className="font-medium text-gray-900 dark:text-gray-100">{viewingClient.phone || "-"}</p>
              </div>
              <div>
                <p className="text-sm text-gray-500 dark:text-gray-400">{t("fields.city")}</p>
                <p className="font-medium text-gray-900 dark:text-gray-100">{viewingClient.city || "-"}</p>
              </div>
              {viewingClient.address && (
                <div className="col-span-2">
                  <p className="text-sm text-gray-500 dark:text-gray-400">{t("fields.address")}</p>
                  <p className="font-medium text-gray-900 dark:text-gray-100">
                    {viewingClient.address}
                    {viewingClient.postal_code && `, ${viewingClient.postal_code}`}
                    {viewingClient.city && ` ${viewingClient.city}`}
                  </p>
                </div>
              )}
            </div>

            <div className="border-t border-gray-200 dark:border-gray-700 pt-4">
              <div className="flex items-center gap-2 mb-4">
                <Users className="h-5 w-5 text-gray-500 dark:text-gray-400" />
                <h3 className="font-medium text-gray-900 dark:text-gray-100">{t("contacts.title")}</h3>
              </div>
              <ClientContacts clientId={viewingClient.id} />
            </div>

            <div className="flex justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
              <Button variant="secondary" onClick={() => setViewingClient(null)}>
                {tCommon("buttons.close")}
              </Button>
              <Button
                onClick={() => {
                  setViewingClient(null);
                  handleOpenModal(viewingClient);
                }}
              >
                <Pencil className="h-4 w-4 mr-2" />
                {t("editClient")}
              </Button>
            </div>
          </div>
        )}
      </Modal>
    </div>
  );
}
