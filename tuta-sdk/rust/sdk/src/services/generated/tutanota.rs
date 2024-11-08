#![allow(unused_imports, dead_code, unused_variables)]
use crate::entities::generated::tutanota::CalendarDeleteData;
use crate::entities::generated::tutanota::CreateGroupPostReturn;
use crate::entities::generated::tutanota::CreateMailFolderData;
use crate::entities::generated::tutanota::CreateMailFolderReturn;
use crate::entities::generated::tutanota::CreateMailGroupData;
use crate::entities::generated::tutanota::CustomerAccountCreateData;
use crate::entities::generated::tutanota::DeleteGroupData;
use crate::entities::generated::tutanota::DeleteMailData;
use crate::entities::generated::tutanota::DeleteMailFolderData;
use crate::entities::generated::tutanota::DraftCreateData;
use crate::entities::generated::tutanota::DraftCreateReturn;
use crate::entities::generated::tutanota::DraftUpdateData;
use crate::entities::generated::tutanota::DraftUpdateReturn;
use crate::entities::generated::tutanota::EncryptTutanotaPropertiesData;
use crate::entities::generated::tutanota::EntropyData;
use crate::entities::generated::tutanota::ExternalUserData;
use crate::entities::generated::tutanota::GroupInvitationDeleteData;
use crate::entities::generated::tutanota::GroupInvitationPostData;
use crate::entities::generated::tutanota::GroupInvitationPostReturn;
use crate::entities::generated::tutanota::GroupInvitationPutData;
use crate::entities::generated::tutanota::ListUnsubscribeData;
use crate::entities::generated::tutanota::MoveMailData;
use crate::entities::generated::tutanota::NewsIn;
use crate::entities::generated::tutanota::NewsOut;
use crate::entities::generated::tutanota::ReceiveInfoServiceData;
use crate::entities::generated::tutanota::ReportMailPostData;
use crate::entities::generated::tutanota::SendDraftData;
use crate::entities::generated::tutanota::SendDraftReturn;
use crate::entities::generated::tutanota::SimpleMoveMailPostIn;
use crate::entities::generated::tutanota::TranslationGetIn;
use crate::entities::generated::tutanota::TranslationGetOut;
use crate::entities::generated::tutanota::UnreadMailStatePostIn;
use crate::entities::generated::tutanota::UpdateMailFolderData;
use crate::entities::generated::tutanota::UserAccountCreateData;
use crate::entities::generated::tutanota::UserAreaGroupDeleteData;
use crate::entities::generated::tutanota::UserAreaGroupPostData;
use crate::entities::Entity;
use crate::rest_client::HttpMethod;
use crate::services::hidden::Nothing;
use crate::services::{
	DeleteService, Executor, ExtraServiceParams, GetService, PostService, PutService, Service,
};
use crate::ApiCallError;
pub struct CalendarService;

crate::service_impl!(declare, CalendarService, "tutanota/calendarservice", 76);
crate::service_impl!(
	POST,
	CalendarService,
	UserAreaGroupPostData,
	CreateGroupPostReturn
);
crate::service_impl!(DELETE, CalendarService, CalendarDeleteData, ());

pub struct ContactListGroupService;

crate::service_impl!(
	declare,
	ContactListGroupService,
	"tutanota/contactlistgroupservice",
	76
);
crate::service_impl!(
	POST,
	ContactListGroupService,
	UserAreaGroupPostData,
	CreateGroupPostReturn
);
crate::service_impl!(DELETE, ContactListGroupService, UserAreaGroupDeleteData, ());

pub struct CustomerAccountService;

crate::service_impl!(
	declare,
	CustomerAccountService,
	"tutanota/customeraccountservice",
	76
);
crate::service_impl!(POST, CustomerAccountService, CustomerAccountCreateData, ());

pub struct DraftService;

crate::service_impl!(declare, DraftService, "tutanota/draftservice", 76);
crate::service_impl!(POST, DraftService, DraftCreateData, DraftCreateReturn);
crate::service_impl!(PUT, DraftService, DraftUpdateData, DraftUpdateReturn);

pub struct EncryptTutanotaPropertiesService;

crate::service_impl!(
	declare,
	EncryptTutanotaPropertiesService,
	"tutanota/encrypttutanotapropertiesservice",
	76
);
crate::service_impl!(
	POST,
	EncryptTutanotaPropertiesService,
	EncryptTutanotaPropertiesData,
	()
);

pub struct EntropyService;

crate::service_impl!(declare, EntropyService, "tutanota/entropyservice", 76);
crate::service_impl!(PUT, EntropyService, EntropyData, ());

pub struct ExternalUserService;

crate::service_impl!(
	declare,
	ExternalUserService,
	"tutanota/externaluserservice",
	76
);
crate::service_impl!(POST, ExternalUserService, ExternalUserData, ());

pub struct GroupInvitationService;

crate::service_impl!(
	declare,
	GroupInvitationService,
	"tutanota/groupinvitationservice",
	76
);
crate::service_impl!(
	POST,
	GroupInvitationService,
	GroupInvitationPostData,
	GroupInvitationPostReturn
);
crate::service_impl!(PUT, GroupInvitationService, GroupInvitationPutData, ());
crate::service_impl!(
	DELETE,
	GroupInvitationService,
	GroupInvitationDeleteData,
	()
);

pub struct ListUnsubscribeService;

crate::service_impl!(
	declare,
	ListUnsubscribeService,
	"tutanota/listunsubscribeservice",
	76
);
crate::service_impl!(POST, ListUnsubscribeService, ListUnsubscribeData, ());

pub struct MailFolderService;

crate::service_impl!(declare, MailFolderService, "tutanota/mailfolderservice", 76);
crate::service_impl!(
	POST,
	MailFolderService,
	CreateMailFolderData,
	CreateMailFolderReturn
);
crate::service_impl!(PUT, MailFolderService, UpdateMailFolderData, ());
crate::service_impl!(DELETE, MailFolderService, DeleteMailFolderData, ());

pub struct MailGroupService;

crate::service_impl!(declare, MailGroupService, "tutanota/mailgroupservice", 76);
crate::service_impl!(POST, MailGroupService, CreateMailGroupData, ());
crate::service_impl!(DELETE, MailGroupService, DeleteGroupData, ());

pub struct MailService;

crate::service_impl!(declare, MailService, "tutanota/mailservice", 76);
crate::service_impl!(DELETE, MailService, DeleteMailData, ());

pub struct MoveMailService;

crate::service_impl!(declare, MoveMailService, "tutanota/movemailservice", 76);
crate::service_impl!(POST, MoveMailService, MoveMailData, ());

pub struct NewsService;

crate::service_impl!(declare, NewsService, "tutanota/newsservice", 76);
crate::service_impl!(POST, NewsService, NewsIn, ());
crate::service_impl!(GET, NewsService, (), NewsOut);

pub struct ReceiveInfoService;

crate::service_impl!(
	declare,
	ReceiveInfoService,
	"tutanota/receiveinfoservice",
	76
);
crate::service_impl!(POST, ReceiveInfoService, ReceiveInfoServiceData, ());

pub struct ReportMailService;

crate::service_impl!(declare, ReportMailService, "tutanota/reportmailservice", 76);
crate::service_impl!(POST, ReportMailService, ReportMailPostData, ());

pub struct SendDraftService;

crate::service_impl!(declare, SendDraftService, "tutanota/senddraftservice", 76);
crate::service_impl!(POST, SendDraftService, SendDraftData, SendDraftReturn);

pub struct SimpleMoveMailService;

crate::service_impl!(
	declare,
	SimpleMoveMailService,
	"tutanota/simplemovemailservice",
	76
);
crate::service_impl!(POST, SimpleMoveMailService, SimpleMoveMailPostIn, ());

pub struct TemplateGroupService;

crate::service_impl!(
	declare,
	TemplateGroupService,
	"tutanota/templategroupservice",
	76
);
crate::service_impl!(
	POST,
	TemplateGroupService,
	UserAreaGroupPostData,
	CreateGroupPostReturn
);
crate::service_impl!(DELETE, TemplateGroupService, UserAreaGroupDeleteData, ());

pub struct TranslationService;

crate::service_impl!(
	declare,
	TranslationService,
	"tutanota/translationservice",
	76
);
crate::service_impl!(GET, TranslationService, TranslationGetIn, TranslationGetOut);

pub struct UnreadMailStateService;

crate::service_impl!(
	declare,
	UnreadMailStateService,
	"tutanota/unreadmailstateservice",
	76
);
crate::service_impl!(POST, UnreadMailStateService, UnreadMailStatePostIn, ());

pub struct UserAccountService;

crate::service_impl!(
	declare,
	UserAccountService,
	"tutanota/useraccountservice",
	76
);
crate::service_impl!(POST, UserAccountService, UserAccountCreateData, ());