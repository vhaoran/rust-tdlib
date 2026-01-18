//use mongodb::bson::oid;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MessageProperties {
    // bool 	can_add_offer_
    // True, if an offer can be added to the message using addOffer.
    #[serde(default)]
    can_add_offer: bool,

    // bool 	can_be_copied_to_secret_chat_
    // True, if content of the message can be copied to a secret chat using inputMessageForwarded or forwardMessages with copy options.
    #[serde(default)]
    can_be_copied_to_secret_chat: bool,

    // bool 	can_be_declined_
    // True, if the message is a suggested post that can be declined by the user using declineSuggestedPost.
    #[serde(default)]
    can_be_declined: bool,

    // bool 	can_be_deleted_only_for_self_
    // True, if the message can be deleted only for the current user while other users will continue to see it using the method deleteMessages with revoke == false.
    #[serde(default)]
    can_be_deleted_only_for_self: bool,

    // bool 	can_be_deleted_for_all_users_
    // True, if the message can be deleted for all users using the method deleteMessages with revoke == true.
    #[serde(default)]
    can_be_deleted_for_all_users: bool,

    // bool 	can_be_edited_
    // True, if the message can be edited using the methods editMessageText, editMessageCaption, or editMessageReplyMarkup. For live location, poll, and checklist messages this fields shows whether editMessageLiveLocation, stopPoll, or editMessageChecklist respectively can be used with this message.
    #[serde(default)]
    can_be_edited: bool,

    // bool 	can_be_forwarded_
    // True, if the message can be forwarded using inputMessageForwarded or forwardMessages without copy options.
    #[serde(default)]
    can_be_forwarded: bool,

    // bool 	can_be_paid_
    // True, if the message can be paid using inputInvoiceMessage.
    #[serde(default)]
    can_be_paid: bool,

    // bool 	can_be_pinned_
    // True, if the message can be pinned or unpinned in the chat using pinChatMessage or unpinChatMessage.
    #[serde(default)]
    can_be_pinned: bool,

    // bool 	can_be_replied_
    // True, if the message can be replied in the same chat and forum topic using inputMessageReplyToMessage.
    #[serde(default)]
    can_be_replied: bool,

    // bool 	can_be_replied_in_another_chat_
    // True, if the message can be replied in another chat or forum topic using inputMessageReplyToExternalMessage.
    #[serde(default)]
    can_be_replied_in_another_chat: bool,

    // bool 	can_be_saved_
    // True, if content of the message can be saved locally.
    #[serde(default)]
    can_be_saved: bool,

    // bool 	can_be_shared_in_story_
    #[serde(default)]
    can_be_shared_in_story: bool,

    // bool 	can_edit_media_
    // True, if the message can be edited using the method editMessageMedia.
    #[serde(default)]
    can_edit_media: bool,

    // bool 	can_edit_scheduling_state_
    // True, if scheduling state of the message can be edited.
    #[serde(default)]
    can_edit_scheduling_state: bool,

    // bool 	can_edit_suggested_post_info_
    // True, if another price or post send time can be suggested using addOffer.
    #[serde(default)]
    can_edit_suggested_post_info: bool,

    // bool 	can_get_author_
    // True, if author of the message sent on behalf of a chat can be received through getMessageAuthor.
    #[serde(default)]
    can_get_author: bool,

    // bool 	can_get_embedding_code_
    // True, if code for message embedding can be received using getMessageEmbeddingCode.
    #[serde(default)]
    can_get_embedding_code: bool,

    // bool 	can_get_link_
    // True, if a link can be generated for the message using getMessageLink.
    #[serde(default)]
    can_get_link: bool,

    // bool 	can_get_media_timestamp_links_
    // True, if media timestamp links can be generated for media timestamp entities in the message text, caption or link preview description using getMessageLink.
    #[serde(default)]
    can_get_media_timestamp_links: bool,

    // bool 	can_get_message_thread_
    // True, if information about the message thread is available through getMessageThread and getMessageThreadHistory.
    #[serde(default)]
    can_get_message_thread: bool,

    // bool 	can_get_read_date_
    // True, if read date of the message can be received through getMessageReadDate.
    #[serde(default)]
    can_get_read_date: bool,

    // bool 	can_get_statistics_
    // True, if message statistics are available through getMessageStatistics and message forwards can be received using getMessagePublicForwards.
    #[serde(default)]
    can_get_statistics: bool,

    // bool 	can_get_video_advertisements_
    // True, if advertisements for video of the message can be received though getVideoMessageAdvertisements.
    #[serde(default)]
    can_get_video_advertisements: bool,

    // bool 	can_get_viewers_
    // True, if chat members already viewed the message can be received through getMessageViewers.
    #[serde(default)]
    can_get_viewers: bool,

    // bool 	can_mark_tasks_as_done_
    // True, if tasks can be marked as done or not done in the message's checklist using markChecklistTasksAsDone if the current user has Telegram Premium subscription.
    #[serde(default)]
    can_mark_tasks_as_done: bool,

    // bool 	can_recognize_speech_
    // True, if speech can be recognized for the message through recognizeSpeech.
    #[serde(default)]
    can_recognize_speech: bool,

    // bool 	can_report_chat_
    // True, if the message can be reported using reportChat.
    //
    #[serde(default)]
    can_report_reactions: bool,

    // bool 	can_report_supergroup_spam_
    // True, if the message can be reported using reportSupergroupSpam.
    #[serde(default)]
    can_report_supergroup_spam: bool,

    // bool 	can_set_fact_check_
    // True, if fact check for the message can be changed through setMessageFactCheck.
    #[serde(default)]
    can_set_fact_check: bool,

    // bool 	need_show_statistics_
    // True, if message statistics must be available from context menu of the message.
    #[serde(default)]
    need_show_statistics: bool,

    #[serde(default)]
    can_add_offset: bool,
    #[serde(default)]
    can_add_tasks: bool,
    #[serde(default)]
    can_be_approved: bool,
    #[serde(default)]
    can_be_copied: bool,

    #[serde(default)]
    can_get_added_reactions: bool,
    #[serde(default)]
    can_report_chat: bool,
}

impl MessageProperties {
    pub fn can_be_deleted_only_for_self(&self) -> bool {
        self.can_be_deleted_only_for_self
    }
    pub fn can_be_deleted_for_all_users(&self) -> bool {
        self.can_be_deleted_for_all_users
    }
    pub fn can_be_edited(&self) -> bool {
        self.can_be_edited
    }
    pub fn can_be_forwarded(&self) -> bool {
        self.can_be_forwarded
    }
    pub fn can_be_paid(&self) -> bool {
        self.can_be_paid
    }
    pub fn can_be_pinned(&self) -> bool {
        self.can_be_pinned
    }
    pub fn can_be_replied(&self) -> bool {
        self.can_be_replied
    }
    pub fn can_be_replied_in_another_chat(&self) -> bool {
        self.can_be_replied_in_another_chat
    }
    pub fn can_be_saved(&self) -> bool {
        self.can_be_saved
    }
    pub fn can_be_shared_in_story(&self) -> bool {
        self.can_be_shared_in_story
    }
    pub fn can_edit_scheduling_state(&self) -> bool {
        self.can_edit_scheduling_state
    }
    pub fn can_get_added_reactions(&self) -> bool {
        self.can_get_added_reactions
    }
    pub fn can_get_embedding_code(&self) -> bool {
        self.can_get_embedding_code
    }
    pub fn can_get_link(&self) -> bool {
        self.can_get_link
    }
    pub fn can_get_media_timestamp_links(&self) -> bool {
        self.can_get_media_timestamp_links
    }
    pub fn can_get_message_thread(&self) -> bool {
        self.can_get_message_thread
    }
    pub fn can_get_read_date(&self) -> bool {
        self.can_get_read_date
    }
    pub fn can_get_statistics(&self) -> bool {
        self.can_get_statistics
    }
    pub fn can_get_viewers(&self) -> bool {
        self.can_get_viewers
    }
    pub fn can_recognize_speech(&self) -> bool {
        self.can_recognize_speech
    }
    pub fn can_report_chat(&self) -> bool {
        self.can_report_chat
    }
    pub fn can_report_reactions(&self) -> bool {
        self.can_report_reactions
    }
    pub fn can_report_supergroup_spam(&self) -> bool {
        self.can_report_supergroup_spam
    }
    pub fn can_set_fact_check(&self) -> bool {
        self.can_set_fact_check
    }
    pub fn need_show_statistics(&self) -> bool {
        self.need_show_statistics
    }
    pub fn can_add_offset(&self) -> bool {
        self.can_add_offset
    }
    pub fn can_add_tasks(&self) -> bool {
        self.can_add_tasks
    }
    pub fn can_be_approved(&self) -> bool {
        self.can_be_approved
    }
    pub fn can_be_copied(&self) -> bool {
        self.can_be_copied
    }
}
