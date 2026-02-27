import { useForm, type Resolver } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useTranslation } from "react-i18next";
import { Save, Upload, Download, AlertCircle, Image, Trash2, Clock, Sun, Moon, Monitor, Globe, HardDrive, FolderOpen, RefreshCw, Lock, Eye, EyeOff, Users, KeyRound } from "lucide-react";
import { LicenseSettingsSection } from "@/features/licensing/LicenseSettingsSection";
import { isTauri } from "@/lib/config";
import { toast } from "@/stores/useToastStore";
import { useAuthStore } from "@/stores/useAuthStore";
import { UserManagement } from "./components/UserManagement";
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardFooter,
  Input,
  Textarea,
  Select,
  Modal,
} from "@/components/ui";
import {
  useCompanySettings,
  useUpdateCompanySettings,
  useUpdateAppSettings,
  useExportBackup,
  useImportBackup,
  useUploadLogo,
  useLogoBase64,
  useDeleteLogo,
  useCreateLocalBackup,
  useOpenBackupsFolder,
  useUpdateBackupSettings,
} from "./hooks/useSettings";
import { useSettingsStore, type AppLanguage, type AppTheme } from "@/stores/useSettingsStore";
import { useEffect, useState } from "react";
import { useLicenseStore } from "@/stores/useLicenseStore";

const createSettingsSchema = (t: (key: string) => string) => z.object({
  company_name: z.string().min(1, t("validation.companyNameRequired")),
  address: z.string().nullable().optional(),
  city: z.string().nullable().optional(),
  postal_code: z.string().nullable().optional(),
  country: z.string().nullable().optional(),
  phone: z.string().nullable().optional(),
  email: z.string().email(t("validation.emailInvalid")).nullable().optional(),
  website: z.string().nullable().optional(),
  siret: z.string().nullable().optional(),
  vat_number: z.string().nullable().optional(),
  default_vat_rate: z.coerce.number().min(0).max(100),
  default_payment_terms: z.coerce.number().min(0),
  invoice_prefix: z.string().min(1, t("validation.invoicePrefixRequired")),
  quote_prefix: z.string().min(1, t("validation.quotePrefixRequired")),
  delivery_note_prefix: z.string().min(1, t("validation.deliveryNotePrefixRequired")).nullable().optional(),
  legal_mentions: z.string().nullable().optional(),
  bank_details: z.string().nullable().optional(),
  currency: z.string().optional().nullable(),
});

type SettingsFormData = z.output<ReturnType<typeof createSettingsSchema>>;

const vatRateOptions = [
  { value: "0", label: "0%" },
  { value: "5.5", label: "5.5%" },
  { value: "10", label: "10%" },
  { value: "20", label: "20%" },
];

const currencyOptions = [
  { value: "EUR", label: "EUR - Euro (\u20AC)" },
  { value: "USD", label: "USD - US Dollar ($)" },
  { value: "GBP", label: "GBP - British Pound (\u00A3)" },
  { value: "DZD", label: "DZD - Dinar alg\u00E9rien (\u062F.\u062C)" },
  { value: "MAD", label: "MAD - Dirham marocain (\u062F.\u0645.)" },
  { value: "TND", label: "TND - Dinar tunisien (\u062F.\u062A)" },
  { value: "CAD", label: "CAD - Dollar canadien (CA$)" },
  { value: "CHF", label: "CHF - Franc suisse (CHF)" },
];

export function SettingsPage() {
  const { t, i18n } = useTranslation("settings");
  const canWrite = useLicenseStore(s => s.isWriteAllowed);
  const settingsSchema = createSettingsSchema(t);
  const { data: settings, isLoading } = useCompanySettings();
  const { data: logoBase64 } = useLogoBase64();
  const updateSettings = useUpdateCompanySettings();
  const uploadLogo = useUploadLogo();
  const deleteLogo = useDeleteLogo();
  const exportBackup = useExportBackup();
  const importBackup = useImportBackup();
  const createLocalBackup = useCreateLocalBackup();
  const openBackupsFolder = useOpenBackupsFolder();
  const updateAppSettings = useUpdateAppSettings();
  const updateBackupSettings = useUpdateBackupSettings();
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [backupSuccess, setBackupSuccess] = useState(false);

  // Password modal states
  const [exportModalOpen, setExportModalOpen] = useState(false);
  const [importModalOpen, setImportModalOpen] = useState(false);
  const [pendingExportPath, setPendingExportPath] = useState<string | null>(null);
  const [pendingImportPath, setPendingImportPath] = useState<string | null>(null);
  const [backupPassword, setBackupPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [passwordError, setPasswordError] = useState("");

  // Theme and language settings
  const { language, theme, setLanguage, setTheme, setCurrency } = useSettingsStore();
  const { currentUser } = useAuthStore();

  const autoUpdateEnabled = settings?.auto_update_enabled ?? true;

  const handleLanguageChange = async (newLang: string) => {
    setLanguage(newLang as AppLanguage);
    // Persist to database (ThemeProvider handles i18n sync reactively)
    await updateAppSettings.mutateAsync({ appLanguage: newLang, appTheme: theme, autoUpdateEnabled });
  };

  const handleThemeChange = async (newTheme: AppTheme) => {
    setTheme(newTheme);
    // Persist to database
    await updateAppSettings.mutateAsync({ appLanguage: language, appTheme: newTheme, autoUpdateEnabled });
  };

  const handleAutoUpdateToggle = async () => {
    const newValue = !autoUpdateEnabled;
    await updateAppSettings.mutateAsync({ appLanguage: language, appTheme: theme, autoUpdateEnabled: newValue });
  };

  const handleUploadLogo = async () => {
    if (!isTauri()) return;
    const { open } = await import("@tauri-apps/plugin-dialog");
    const filePath = await open({
      multiple: false,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg", "gif", "webp"],
        },
      ],
    });
    if (filePath) {
      await uploadLogo.mutateAsync(filePath);
    }
  };

  const handleDeleteLogo = async () => {
    if (confirm(t("messages.deleteLogoConfirm"))) {
      await deleteLogo.mutateAsync();
    }
  };

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isDirty },
  } = useForm<SettingsFormData>({
    resolver: zodResolver(settingsSchema) as Resolver<SettingsFormData>,
    defaultValues: {
      company_name: "",
      address: "",
      city: "",
      postal_code: "",
      country: "France",
      phone: "",
      email: "",
      website: "",
      siret: "",
      vat_number: "",
      default_vat_rate: 0,
      default_payment_terms: 30,
      invoice_prefix: "FA-",
      quote_prefix: "DE-",
      delivery_note_prefix: "BL-",
      legal_mentions: "",
      bank_details: "",
      currency: "EUR",
    },
  });

  useEffect(() => {
    if (settings) {
      reset({
        company_name: settings.company_name,
        address: settings.address ?? "",
        city: settings.city ?? "",
        postal_code: settings.postal_code ?? "",
        country: settings.country ?? "France",
        phone: settings.phone ?? "",
        email: settings.email ?? "",
        website: settings.website ?? "",
        siret: settings.siret ?? "",
        vat_number: settings.vat_number ?? "",
        default_vat_rate: settings.default_vat_rate,
        default_payment_terms: settings.default_payment_terms,
        invoice_prefix: settings.invoice_prefix,
        quote_prefix: settings.quote_prefix,
        delivery_note_prefix: settings.delivery_note_prefix ?? "BL-",
        legal_mentions: settings.legal_mentions ?? "",
        bank_details: settings.bank_details ?? "",
        currency: settings.currency ?? "EUR",
      });
    }
  }, [settings, reset]);

  const onSubmit = async (data: SettingsFormData) => {
    await updateSettings.mutateAsync(data);
    if (data.currency) {
      setCurrency(data.currency);
    }
    setSaveSuccess(true);
    setTimeout(() => setSaveSuccess(false), 3000);
  };

  const handleExport = async () => {
    if (!isTauri()) return;
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const filePath = await open({
        multiple: false,
        directory: true,
      });
      if (filePath) {
        const backupPath = `${filePath}/probook-backup-${new Date().toISOString().split('T')[0]}.enc`;
        setPendingExportPath(backupPath);
        setBackupPassword("");
        setConfirmPassword("");
        setPasswordError("");
        setExportModalOpen(true);
      }
    } catch (error) {
      toast.error(t("messages.exportFailed"));
    }
  };

  const handleExportConfirm = async () => {
    if (!pendingExportPath) return;

    if (backupPassword.length < 8) {
      setPasswordError(t("backup.passwordTooShort"));
      return;
    }

    if (backupPassword !== confirmPassword) {
      setPasswordError(t("backup.passwordMismatch"));
      return;
    }

    try {
      await exportBackup.mutateAsync({ filePath: pendingExportPath, password: backupPassword });
      setExportModalOpen(false);
      setPendingExportPath(null);
      setBackupPassword("");
      setConfirmPassword("");
      toast.success(t("messages.exportSuccess"));
    } catch (error) {
      setPasswordError(t("messages.exportFailed"));
    }
  };

  const handleImport = async () => {
    if (!isTauri()) return;
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const filePath = await open({
        multiple: false,
        filters: [
          {
            name: "Backup Files",
            extensions: ["enc"],
          },
        ],
      });
      if (filePath) {
        setPendingImportPath(filePath);
        setBackupPassword("");
        setPasswordError("");
        setImportModalOpen(true);
      }
    } catch (error) {
      toast.error(t("messages.importFailed"));
    }
  };

  const handleImportConfirm = async () => {
    if (!pendingImportPath) return;

    try {
      await importBackup.mutateAsync({ filePath: pendingImportPath, password: backupPassword });
      setImportModalOpen(false);
      setPendingImportPath(null);
      setBackupPassword("");
      toast.success(t("messages.importSuccess"));
    } catch (error) {
      setPasswordError(t("backup.wrongPassword"));
    }
  };

  const handleBackupNow = async () => {
    try {
      await createLocalBackup.mutateAsync();
      setBackupSuccess(true);
      setTimeout(() => setBackupSuccess(false), 3000);
    } catch (error) {
      toast.error(t("messages.backupFailed"));
    }
  };

  const handleOpenBackupsFolder = async () => {
    try {
      await openBackupsFolder.mutateAsync();
    } catch (error) {
      toast.error(t("messages.openFolderFailed"));
    }
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
      <div>
        <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-100">{t("title")}</h1>
        <p className="text-gray-500 dark:text-gray-400">{t("subtitle")}</p>
      </div>

      {/* License */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <KeyRound className="h-5 w-5" />
            {t("license.title")}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <LicenseSettingsSection />
        </CardContent>
      </Card>

      {/* User Management (Admin only) */}
      {currentUser?.role === 'admin' && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Users className="h-5 w-5" />
              {t("userManagement.title", { ns: "auth" })}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <UserManagement />
          </CardContent>
        </Card>
      )}

      {/* Appearance Settings */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Globe className="h-5 w-5" />
            {t("appearance.title")}
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Language Selection */}
          <div>
            <span className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
              {t("appearance.language")}
            </span>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
              {[
                { value: 'system', label: t("appearance.systemLanguage"), flag: <Globe className="h-5 w-5" /> },
                { value: 'fr', label: 'Français', flag: (
                  <svg className="h-5 w-5 rounded-sm" viewBox="0 0 640 480">
                    <rect width="213.3" height="480" fill="#002654"/>
                    <rect x="213.3" width="213.4" height="480" fill="#fff"/>
                    <rect x="426.7" width="213.3" height="480" fill="#ce1126"/>
                  </svg>
                )},
                { value: 'en', label: 'English', flag: (
                  <svg className="h-5 w-5 rounded-sm" viewBox="0 0 640 480">
                    <rect width="640" height="480" fill="#012169"/>
                    <path d="m75 0 244 181L562 0h78v62L400 241l240 178v61h-80L320 301 81 480H0v-60l239-178L0 64V0h75z" fill="#fff"/>
                    <path d="m424 281 216 159v40L369 281h55zm-184 20 6 35L54 480H0l240-179zM640 0v3L391 191l2-44L590 0h50zM0 0l239 176h-60L0 42V0z" fill="#C8102E"/>
                    <path d="M241 0v480h160V0H241zM0 160v160h640V160H0z" fill="#fff"/>
                    <path d="M0 193v96h640v-96H0zM273 0v480h96V0h-96z" fill="#C8102E"/>
                  </svg>
                )},
                { value: 'ar', label: 'العربية', flag: (
                  <svg className="h-5 w-5 rounded-sm" viewBox="0 0 640 480">
                    <rect width="640" height="480" fill="#006c35"/>
                    <path d="M170 195h300v90H170z" fill="#fff"/>
                    <text x="320" y="270" textAnchor="middle" fill="#fff" fontSize="48" fontFamily="serif">لا إله إلا الله</text>
                  </svg>
                )},
              ].map((lang) => (
                <button
                  key={lang.value}
                  type="button"
                  onClick={() => handleLanguageChange(lang.value)}
                  className={`flex items-center justify-center gap-2 px-4 py-3 rounded-lg border-2 transition-colors ${
                    language === lang.value
                      ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 text-primary-700 dark:text-primary-300'
                      : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 text-gray-700 dark:text-gray-300'
                  }`}
                >
                  <span className="shrink-0">{lang.flag}</span>
                  <span className="text-sm font-medium">{lang.label}</span>
                </button>
              ))}
            </div>
          </div>

          {/* Theme Selection */}
          <div>
            <span className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
              {t("appearance.theme")}
            </span>
            <div className="grid grid-cols-3 gap-2">
              {[
                { value: 'system' as AppTheme, label: t("appearance.systemTheme"), icon: Monitor },
                { value: 'light' as AppTheme, label: t("appearance.lightTheme"), icon: Sun },
                { value: 'dark' as AppTheme, label: t("appearance.darkTheme"), icon: Moon },
              ].map((themeOption) => (
                <button
                  key={themeOption.value}
                  type="button"
                  onClick={() => handleThemeChange(themeOption.value)}
                  className={`flex items-center justify-center gap-2 px-4 py-3 rounded-lg border-2 transition-colors ${
                    theme === themeOption.value
                      ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 text-primary-700 dark:text-primary-300'
                      : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 text-gray-700 dark:text-gray-300'
                  }`}
                >
                  <themeOption.icon className="h-5 w-5" />
                  <span className="text-sm font-medium">{themeOption.label}</span>
                </button>
              ))}
            </div>
          </div>

          {/* Auto-update toggle */}
          <div className="flex items-center justify-between">
            <div>
              <span className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                {t("appearance.autoUpdate")}
              </span>
              <span className="block text-sm text-gray-500 dark:text-gray-400">
                {t("appearance.autoUpdateDescription")}
              </span>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={autoUpdateEnabled}
              onClick={handleAutoUpdateToggle}
              className={`relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 ${
                autoUpdateEnabled ? 'bg-primary-600' : 'bg-gray-200 dark:bg-gray-700'
              }`}
            >
              <span
                className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                  autoUpdateEnabled ? 'translate-x-5' : 'translate-x-0'
                }`}
              />
            </button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("branding.title")}</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center gap-6">
            <div className="w-24 h-24 sm:w-32 sm:h-32 border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg flex items-center justify-center bg-gray-50 dark:bg-gray-800 overflow-hidden">
              {logoBase64 ? (
                <img
                  src={logoBase64}
                  alt={t("branding.logoAlt")}
                  className="w-full h-full object-contain"
                />
              ) : (
                <Image className="h-12 w-12 text-gray-400" />
              )}
            </div>
            <div className="space-y-3">
              <p className="text-sm text-gray-500">
                {t("branding.logoDescription")}
                <br />
                {t("branding.acceptedFormats")}
              </p>
              <div className="flex gap-2">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={handleUploadLogo}
                  isLoading={uploadLogo.isPending}
                >
                  <Upload className="h-4 w-4 mr-2" />
                  {logoBase64 ? t("branding.changeLogo") : t("branding.uploadLogo")}
                </Button>
                {logoBase64 && (
                  <Button
                    type="button"
                    variant="danger"
                    onClick={handleDeleteLogo}
                    isLoading={deleteLogo.isPending}
                  >
                    <Trash2 className="h-4 w-4 mr-2" />
                    {t("branding.delete")}
                  </Button>
                )}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
        <Card>
          <CardHeader>
            <CardTitle>{t("company.title")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Input
                label={t("company.nameRequired")}
                autoComplete="organization"
                {...register("company_name")}
                error={errors.company_name?.message}
              />
              <Input
                label={t("company.email")}
                type="email"
                autoComplete="email"
                {...register("email")}
                error={errors.email?.message}
              />
              <Input
                label={t("company.phone")}
                autoComplete="tel"
                {...register("phone")}
                error={errors.phone?.message}
              />
              <Input
                label={t("company.website")}
                autoComplete="url"
                {...register("website")}
                error={errors.website?.message}
              />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("company.addressTitle")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <Input
              label={t("company.address")}
              autoComplete="street-address"
              {...register("address")}
              error={errors.address?.message}
            />
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <Input
                label={t("company.postalCode")}
                autoComplete="postal-code"
                {...register("postal_code")}
                error={errors.postal_code?.message}
              />
              <Input
                label={t("company.city")}
                autoComplete="address-level2"
                {...register("city")}
                error={errors.city?.message}
              />
              <Input
                label={t("company.country")}
                autoComplete="country-name"
                {...register("country")}
                error={errors.country?.message}
              />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("legal.title")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Input
                label={t("company.siret")}
                {...register("siret")}
                error={errors.siret?.message}
              />
              <Input
                label={t("company.vatNumber")}
                {...register("vat_number")}
                error={errors.vat_number?.message}
              />
            </div>
            <Textarea
              label={t("legal.legalMentions")}
              placeholder={t("legal.legalMentionsPlaceholder")}
              {...register("legal_mentions")}
              error={errors.legal_mentions?.message}
            />
            <Textarea
              label={t("legal.bankDetails")}
              placeholder={t("legal.bankDetailsPlaceholder")}
              {...register("bank_details")}
              error={errors.bank_details?.message}
            />
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("billing.title")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <Select
                label={t("billing.currency")}
                options={currencyOptions}
                {...register("currency")}
              />
              <Select
                label={t("billing.defaultVatRate")}
                options={vatRateOptions}
                {...register("default_vat_rate")}
                error={errors.default_vat_rate?.message}
              />
              <Input
                label={t("billing.defaultPaymentTerms")}
                type="number"
                {...register("default_payment_terms")}
                error={errors.default_payment_terms?.message}
              />
              <Input
                label={t("billing.invoicePrefix")}
                {...register("invoice_prefix")}
                error={errors.invoice_prefix?.message}
              />
              <Input
                label={t("billing.quotePrefix")}
                {...register("quote_prefix")}
                error={errors.quote_prefix?.message}
              />
              <Input
                label={t("billing.deliveryNotePrefix")}
                {...register("delivery_note_prefix")}
                error={errors.delivery_note_prefix?.message}
              />
            </div>
          </CardContent>
          <CardFooter>
            <div className="flex items-center gap-4 w-full">
              {saveSuccess && (
                <span className="text-green-600 text-sm">
                  {t("messages.saveSuccess")}
                </span>
              )}
              <div className="flex-1" />
              <Button type="submit" isLoading={updateSettings.isPending} disabled={!isDirty || !canWrite} disabledReason={!canWrite ? t('licensing:tooltip.writeDisabled') : undefined}>
                <Save className="h-4 w-4 mr-2" />
                {t("buttons.save")}
              </Button>
            </div>
          </CardFooter>
        </Card>
      </form>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <HardDrive className="h-5 w-5" />
            {t("backup.title")}
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            {t("backup.description")}
          </p>

          {/* Last backup status */}
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
            <div className="flex items-center gap-2">
              <Clock className="h-4 w-4 text-gray-500 dark:text-gray-400" />
              <p className="text-sm text-gray-600 dark:text-gray-300">
                {settings?.last_backup_date
                  ? t("backup.lastBackupDate", { date: new Date(settings.last_backup_date).toLocaleString(i18n.language === 'ar' ? 'ar-u-nu-latn' : i18n.language) })
                  : t("backup.noBackupYet")}
              </p>
            </div>
            <div className="flex items-center gap-2">
              {backupSuccess && (
                <span className="text-green-600 dark:text-green-400 text-sm">
                  {t("backup.backupCreated")}
                </span>
              )}
              <Button
                type="button"
                variant="secondary"
                size="sm"
                onClick={handleBackupNow}
                isLoading={createLocalBackup.isPending}
              >
                <RefreshCw className="h-4 w-4 mr-2" />
                {t("backup.backupNow")}
              </Button>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                onClick={handleOpenBackupsFolder}
                isLoading={openBackupsFolder.isPending}
              >
                <FolderOpen className="h-4 w-4 mr-2" />
                {t("backup.viewBackups")}
              </Button>
            </div>
          </div>

          {/* Auto-Backup Settings */}
          <div className="border-t border-gray-200 dark:border-gray-700 pt-4 space-y-3">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  {t("backup.autoBackup")}
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  {t("backup.autoBackupDescription")}
                </p>
              </div>
              <button
                type="button"
                role="switch"
                aria-checked={settings?.auto_backup_enabled ?? false}
                onClick={() => {
                  const newEnabled = !(settings?.auto_backup_enabled ?? false);
                  updateBackupSettings.mutate({
                    autoBackupEnabled: newEnabled,
                    backupSchedule: settings?.backup_schedule || "daily",
                  });
                }}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  settings?.auto_backup_enabled
                    ? "bg-primary-600"
                    : "bg-gray-300 dark:bg-gray-600"
                }`}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    settings?.auto_backup_enabled ? "translate-x-6" : "translate-x-1"
                  }`}
                />
              </button>
            </div>
            {settings?.auto_backup_enabled && (
              <div className="flex items-center gap-3">
                <label htmlFor="backup-schedule" className="text-sm text-gray-600 dark:text-gray-400">
                  {t("backup.schedule")}
                </label>
                <select
                  id="backup-schedule"
                  value={settings?.backup_schedule || "daily"}
                  onChange={(e) => {
                    updateBackupSettings.mutate({
                      autoBackupEnabled: true,
                      backupSchedule: e.target.value,
                    });
                  }}
                  className="px-3 py-1.5 text-sm border rounded-lg shadow-sm bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-primary-500"
                >
                  <option value="daily">{t("backup.daily")}</option>
                  <option value="weekly">{t("backup.weekly")}</option>
                  <option value="monthly">{t("backup.monthly")}</option>
                </select>
              </div>
            )}
          </div>

          {/* Manual Export/Import */}
          <div className="border-t border-gray-200 dark:border-gray-700 pt-4">
            <p className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
              {t("backup.manualBackup")}
            </p>
            <div className="flex gap-3">
              <Button
                type="button"
                variant="secondary"
                onClick={handleExport}
                isLoading={exportBackup.isPending}
              >
                <Download className="h-4 w-4 mr-2" />
                {t("backup.export")}
              </Button>
              <Button
                type="button"
                variant="secondary"
                onClick={handleImport}
                isLoading={importBackup.isPending}
              >
                <Upload className="h-4 w-4 mr-2" />
                {t("backup.import")}
              </Button>
            </div>
          </div>

          <div className="flex items-start gap-3 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
            <AlertCircle className="h-4 w-4 text-blue-600 dark:text-blue-400 mt-0.5 shrink-0" />
            <p className="text-sm text-blue-700 dark:text-blue-400">
              {t("backup.tip")}
            </p>
          </div>
        </CardContent>
      </Card>

      {/* Export Password Modal */}
      <Modal
        isOpen={exportModalOpen}
        onClose={() => {
          setExportModalOpen(false);
          setPendingExportPath(null);
          setBackupPassword("");
          setConfirmPassword("");
          setPasswordError("");
        }}
        title={t("backup.setPassword")}
        size="sm"
      >
        <div className="space-y-4">
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {t("backup.passwordDescription")}
          </p>
          <div className="relative">
            <Input
              label={t("backup.password")}
              type={showPassword ? "text" : "password"}
              value={backupPassword}
              onChange={(e) => setBackupPassword(e.target.value)}
              placeholder={t("backup.passwordPlaceholder")}
              className="pr-10"
            />
            <button
              type="button"
              onClick={() => setShowPassword(!showPassword)}
              className="absolute right-3 top-8.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              aria-label={showPassword ? t("backup.hidePassword") : t("backup.showPassword")}
            >
              {showPassword ? <EyeOff className="h-5 w-5" /> : <Eye className="h-5 w-5" />}
            </button>
          </div>
          <Input
            label={t("backup.confirmPassword")}
            type={showPassword ? "text" : "password"}
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            placeholder={t("backup.confirmPasswordPlaceholder")}
          />
          {passwordError && (
            <p className="text-sm text-red-600 dark:text-red-400">{passwordError}</p>
          )}
          <div className="flex justify-end gap-3 pt-2">
            <Button
              variant="secondary"
              onClick={() => {
                setExportModalOpen(false);
                setPendingExportPath(null);
                setBackupPassword("");
                setConfirmPassword("");
                setPasswordError("");
              }}
            >
              {t("buttons.cancel")}
            </Button>
            <Button
              onClick={handleExportConfirm}
              isLoading={exportBackup.isPending}
            >
              <Lock className="h-4 w-4 mr-2" />
              {t("backup.exportEncrypted")}
            </Button>
          </div>
        </div>
      </Modal>

      {/* Import Password Modal */}
      <Modal
        isOpen={importModalOpen}
        onClose={() => {
          setImportModalOpen(false);
          setPendingImportPath(null);
          setBackupPassword("");
          setPasswordError("");
        }}
        title={t("backup.enterPassword")}
        size="sm"
      >
        <div className="space-y-4">
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {t("backup.importPasswordDescription")}
          </p>
          <div className="relative">
            <Input
              label={t("backup.password")}
              type={showPassword ? "text" : "password"}
              value={backupPassword}
              onChange={(e) => setBackupPassword(e.target.value)}
              placeholder={t("backup.passwordPlaceholder")}
              className="pr-10"
            />
            <button
              type="button"
              onClick={() => setShowPassword(!showPassword)}
              className="absolute right-3 top-8.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              aria-label={showPassword ? t("backup.hidePassword") : t("backup.showPassword")}
            >
              {showPassword ? <EyeOff className="h-5 w-5" /> : <Eye className="h-5 w-5" />}
            </button>
          </div>
          {passwordError && (
            <p className="text-sm text-red-600 dark:text-red-400">{passwordError}</p>
          )}
          <div className="flex justify-end gap-3 pt-2">
            <Button
              variant="secondary"
              onClick={() => {
                setImportModalOpen(false);
                setPendingImportPath(null);
                setBackupPassword("");
                setPasswordError("");
              }}
            >
              {t("buttons.cancel")}
            </Button>
            <Button
              onClick={handleImportConfirm}
              isLoading={importBackup.isPending}
            >
              <Lock className="h-4 w-4 mr-2" />
              {t("backup.importDecrypt")}
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
