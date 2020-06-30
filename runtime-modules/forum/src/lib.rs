// Copyright 2017-2019 Parity Technologies (UK) Ltd.

// This is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

// Copyright 2019 Joystream Contributors

//! # Runtime Example Module
//!
//! <!-- Original author of paragraph: @gavofyork -->
//! The Example: A simple example of a runtime module demonstrating
//! concepts, APIs and structures common to most runtime modules.
//!
//! Run `cargo doc --package runtime-example-module --open` to view this module's documentation.
//!
//! ### Documentation Template:<br>
//! Add heading with custom module name
//!
//! # <INSERT_CUSTOM_MODULE_NAME> Module
//!
//! Add simple description
//!
//! Include the following links that shows what trait needs to be implemented to use the module
//! and the supported dispatchables that are documented in the Call enum.
//!
//! - [`<INSERT_CUSTOM_MODULE_NAME>::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! <!-- Original author of paragraph: Various. See https://github.com/paritytech/substrate-developer-hub/issues/44 -->
//! Short description of module purpose.
//! Links to Traits that should be implemented.
//! What this module is for.
//! What functionality the module provides.
//! When to use the module (use case examples).
//! How it is used.
//! Inputs it uses and the source of each input.
//! Outputs it produces.
//!
//! <!-- Original author of paragraph: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//! <!-- and comment https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! ## Terminology
//!
//! Add terminology used in the custom module. Include concepts, storage items, or actions that you think
//! deserve to be noted to give context to the rest of the documentation or module usage. The author needs to
//! use some judgment about what is included. We don't want a list of every storage item nor types - the user
//! can go to the code for that. For example, "transfer fee" is obvious and should not be included, but
//! "free balance" and "reserved balance" should be noted to give context to the module.
//! Please do not link to outside resources. The reference docs should be the ultimate source of truth.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! ## Goals
//!
//! Add goals that the custom module is designed to achieve.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! ### Scenarios
//!
//! <!-- Original author of paragraph: @Kianenigma. Based on PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! #### <INSERT_SCENARIO_NAME>
//!
//! Describe requirements prior to interacting with the custom module.
//! Describe the process of interacting with the custom module for this scenario and public API functions used.
//!
//! ## Interface
//!
//! ### Supported Origins
//!
//! What origins are used and supported in this module (root, signed, inherent)
//! i.e. root when `ensure_root` used
//! i.e. inherent when `ensure_inherent` used
//! i.e. signed when `ensure_signed` used
//!
//! `inherent` <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: @Kianenigma in comment -->
//! <!-- https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! ### Types
//!
//! Type aliases. Include any associated types and where the user would typically define them.
//!
//! `ExampleType` <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: ??? -->
//!
//!
//! ### Dispatchable Functions
//!
//! <!-- Original author of paragraph: @AmarRSingh & @joepetrowski -->
//!
//! // A brief description of dispatchable functions and a link to the rustdoc with their actual documentation.
//!
//! <b>MUST</b> have link to Call enum
//! <b>MUST</b> have origin information included in function doc
//! <b>CAN</b> have more info up to the user
//!
//! ### Public Functions
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! A link to the rustdoc and any notes about usage in the module, not for specific functions.
//! For example, in the balances module: "Note that when using the publicly exposed functions,
//! you (the runtime developer) are responsible for implementing any necessary checks
//! (e.g. that the sender is the signer) before calling a function that will affect storage."
//!
//! <!-- Original author of paragraph: @AmarRSingh -->
//!
//! It is up to the writer of the respective module (with respect to how much information to provide).
//!
//! #### Public Inspection functions - Immutable (getters)
//!
//! Insert a subheading for each getter function signature
//!
//! ##### `example_getter_name()`
//!
//! What it returns
//! Why, when, and how often to call it
//! When it could panic or error
//! When safety issues to consider
//!
//! #### Public Mutable functions (changing state)
//!
//! Insert a subheading for each setter function signature
//!
//! ##### `example_setter_name(origin, parameter_name: T::ExampleType)`
//!
//! What state it changes
//! Why, when, and how often to call it
//! When it could panic or error
//! When safety issues to consider
//! What parameter values are valid and why
//!
//! ### Storage Items
//!
//! Explain any storage items included in this module
//!
//! ### Digest Items
//!
//! Explain any digest items included in this module
//!
//! ### Inherent Data
//!
//! Explain what inherent data (if any) is defined in the module and any other related types
//!
//! ### Events:
//!
//! Insert events for this module if any
//!
//! ### Errors:
//!
//! Explain what generates errors
//!
//! ## Usage
//!
//! Insert 2-3 examples of usage and code snippets that show how to use <INSERT_CUSTOM_MODULE_NAME> module in a custom module.
//!
//! ### Prerequisites
//!
//! Show how to include necessary imports for <INSERT_CUSTOM_MODULE_NAME> and derive
//! your module configuration trait with the `INSERT_CUSTOM_MODULE_NAME` trait.
//!
//! ```rust
//! // use <INSERT_CUSTOM_MODULE_NAME>;
//!
//! // pub trait Trait: <INSERT_CUSTOM_MODULE_NAME>::Trait { }
//! ```
//!
//! ### Simple Code Snippet
//!
//! Show a simple example (e.g. how to query a public getter function of <INSERT_CUSTOM_MODULE_NAME>)
//!
//! ## Genesis Config
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! ## Dependencies
//!
//! Dependencies on other SRML modules and the genesis config should be mentioned,
//! but not the Rust Standard Library.
//! Genesis configuration modifications that may be made to incorporate this module
//! Interaction with other modules
//!
//! <!-- Original author of heading: @AmarRSingh -->
//!
//! ## Related Modules
//!
//! Interaction with other modules in the form of a bullet point list
//!
//! ## References
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! Links to reference material, if applicable. For example, Phragmen, W3F research, etc.
//! that the implementation is based on.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

#[cfg(feature = "std")]
use serde_derive::{Deserialize, Serialize};

use codec::{Codec, Decode, Encode};
use rstd::prelude::*;
pub use runtime_io::clear_prefix;
use runtime_primitives::traits::{MaybeSerialize, Member, One, SimpleArithmetic};
use srml_support::{decl_event, decl_module, decl_storage, dispatch, ensure, Parameter};

mod mock;
mod tests;

pub trait Trait: system::Trait + timestamp::Trait + Sized {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type ForumUserId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    type ModeratorId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    type CategoryId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq
        + From<u64>
        + Into<u64>;

    type ThreadId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq
        + From<u64>
        + Into<u64>;

    type PostId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq
        + From<u64>
        + Into<u64>;

    fn is_lead(account_id: &<Self as system::Trait>::AccountId) -> bool;
    fn is_forum_member(
        account_id: &<Self as system::Trait>::AccountId,
        forum_user_id: &Self::ForumUserId,
    ) -> bool;
    fn is_moderator(account_id: &Self::AccountId, moderator_id: &Self::ModeratorId) -> bool;
}

/*
 * MOVE ALL OF THESE OUT TO COMMON LATER
 */

/// Length constraint for input validation
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct InputValidationLengthConstraint {
    /// Minimum length
    pub min: u16,

    /// Difference between minimum length and max length.
    /// While having max would have been more direct, this
    /// way makes max < min unrepresentable semantically,
    /// which is safer.
    pub max_min_diff: u16,
}

impl InputValidationLengthConstraint {
    /// Helper for computing max
    pub fn max(&self) -> u16 {
        self.min + self.max_min_diff
    }

    pub fn ensure_valid(
        &self,
        len: usize,
        too_short_msg: &'static str,
        too_long_msg: &'static str,
    ) -> Result<(), &'static str> {
        let length = len as u16;
        if length < self.min {
            Err(too_short_msg)
        } else if length > self.max() {
            Err(too_long_msg)
        } else {
            Ok(())
        }
    }
}

/// Error about users
const ERROR_ORIGIN_NOT_FORUM_LEAD: &str = "Origin not forum lead.";
const ERROR_FORUM_USER_ID_NOT_MATCH_ACCOUNT: &str = "Forum user id not match its account.";
const ERROR_MODERATOR_ID_NOT_MATCH_ACCOUNT: &str = "Moderator id not match its account.";

// Errors about thread.
const ERROR_THREAD_DOES_NOT_EXIST: &str = "Thread does not exist";
const ERROR_THREAD_ALREADY_MODERATED: &str = "Thread already moderated.";
const ERROR_THREAD_MODERATED: &str = "Thread is moderated.";
const ERROR_THREAD_WITH_WRONG_CATEGORY_ID: &str = "thread and its category not match.";

// Errors about post.
const ERROR_POST_DOES_NOT_EXIST: &str = "Post does not exist.";
const ERROR_POST_MODERATED: &str = "Post is moderated.";

// Errors about category.
const ERROR_CATEGORY_NOT_BEING_UPDATED: &str = "Category not being updated.";
const ERROR_CATEGORY_CANNOT_BE_UNARCHIVED_WHEN_DELETED: &str =
    "Category cannot be unarchived when deleted.";
const ERROR_MODERATOR_MODERATE_CATEGORY: &str = "Moderator can not moderate category.";
const ERROR_EXCEED_MAX_CATEGORY_DEPTH: &str = "Category exceed max depth.";
const ERROR_ANCESTOR_CATEGORY_IMMUTABLE: &str =
    "Ancestor category immutable, i.e. deleted or archived";
const ERROR_MAX_VALID_CATEGORY_DEPTH_EXCEEDED: &str = "Maximum valid category depth exceeded.";
const ERROR_CATEGORY_DOES_NOT_EXIST: &str = "Category does not exist.";

// Errors about poll.
const ERROR_POLL_ALTERNATIVES_TOO_SHORT: &str = "Poll items number too short.";
const ERROR_POLL_ALTERNATIVES_TOO_LONG: &str = "Poll items number too long.";
const ERROR_POLL_NOT_EXIST: &str = "Poll not exist.";
const ERROR_POLL_TIME_SETTING: &str = "Poll date setting is wrong.";
const ERROR_POLL_DATA: &str = "Poll data committed is wrong.";
const ERROR_POLL_COMMIT_EXPIRED: &str = "Poll data committed after poll expired.";

// Error data migration
const ERROR_DATA_MIGRATION_NOT_DONE: &str = "data migration not done yet.";

//use srml_support::storage::*;
//use sr_io::{StorageOverlay, ChildrenStorageOverlay};
//#[cfg(feature = "std")]
//use runtime_io::{StorageOverlay, ChildrenStorageOverlay};
//#[cfg(any(feature = "std", test))]
//use sr_primitives::{StorageOverlay, ChildrenStorageOverlay};

use system::ensure_signed;

/// Represents a moderator in this forum.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Moderator<AccountId> {
    /// Moderator's account used for extrinsic
    pub role_account: AccountId,

    /// Moderator's name
    pub name: Vec<u8>,

    /// Moderator's self introduction.
    pub self_introduction: Vec<u8>,
}

/// Represents a moderation outcome applied to a post or a thread.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct ModerationAction<ModeratorId> {
    /// Account forum lead which acted.
    pub moderator_id: ModeratorId,
}

/// Represents a reaction to a post
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Copy, Debug)]
pub enum PostReaction {
    /// No any reaction
    NonReacton,

    /// Thumb up to a post
    ThumbUp,

    /// Thumb down to a post
    ThumbDown,

    /// Like a post
    Like,
}

/// Implement default trait for PostReaction
impl Default for PostReaction {
    /// Set default value for PostReaction
    fn default() -> PostReaction {
        Self::NonReacton
    }
}

/// Represents all poll alternatives and vote count for each one
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct PollAlternative<Hash> {
    /// Unique identifier
    pub hash: Hash,

    /// Vote count for the alternative
    pub vote_count: u32,
}

/// Represents a poll
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Poll<Timestamp, Hash> {
    /// Unique identifier
    pub hash: Hash,

    /// timestamp of poll start
    pub start_time: Timestamp,

    /// timestamp of poll end
    pub end_time: Timestamp,

    /// Alternative description and count
    pub poll_alternatives: Vec<PollAlternative<Hash>>,
}

/// Represents a thread post
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Post<ForumUserId, ModeratorId, ThreadId, Hash> {
    /// Id of thread to which this post corresponds.
    pub thread_id: ThreadId,

    /// Unique identifier
    pub hash: Hash,

    /// Possible moderation of this post
    pub moderation: Option<ModerationAction<ModeratorId>>,

    /// Author of post.
    pub author_id: ForumUserId,

    /// The post number of this post in its thread, i.e. total number of posts added (including this)
    /// to a thread when it was added.
    /// Is needed to give light clients assurance about getting all posts in a given range,
    // `created_at` is not sufficient.
    /// Starts at 1 for first post in thread.
    pub nr_in_thread: u32,
}

/// Represents a thread
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Thread<ForumUserId, ModeratorId, CategoryId, Moment, Hash> {
    /// Unique identifier
    pub hash: Hash,

    /// Category in which this thread lives
    pub category_id: CategoryId,

    /// Possible moderation of this thread
    pub moderation: Option<ModerationAction<ModeratorId>>,

    /// Author of post.
    pub author_id: ForumUserId,

    /// poll description.
    pub poll: Option<Poll<Moment, Hash>>,

    /// The thread number of this thread in its category, i.e. total number of thread added (including this)
    /// to a category when it was added.
    /// Is needed to give light clients assurance about getting all threads in a given range,
    /// `created_at` is not sufficient.
    /// Starts at 1 for first thread in category.
    pub nr_in_category: u32,

    /// Number of unmoderated and moderated posts in this thread.
    /// The sum of these two only increases, and former is incremented
    /// for each new post added to this thread. A new post is added
    /// with a `nr_in_thread` equal to this sum
    ///
    /// When there is a moderation
    /// of a post, the variables are incremented and decremented, respectively.
    ///
    /// These values are vital for light clients, in order to validate that they are
    /// not being censored from posts in a thread.
    pub num_unmoderated_posts: u32,
    pub num_moderated_posts: u32,
}

/// Implement total posts calculation for thread
impl<ForumUserId, ModeratorId, CategoryId, Moment, Hash>
    Thread<ForumUserId, ModeratorId, CategoryId, Moment, Hash>
{
    /// How many posts created both unmoderated and moderated
    pub fn num_posts_ever_created(&self) -> u32 {
        self.num_unmoderated_posts + self.num_moderated_posts
    }
}

/// Represents child category position in parent.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct ChildPositionInParentCategory<CategoryId> {
    /// Id of parent category
    pub parent_id: CategoryId,

    /// Nr of the child in the parent
    /// Starts at 1
    pub child_nr_in_parent_category: u32,
}

/// Represents a category
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Category<CategoryId, ThreadId, Hash> {
    /// Category identifier
    pub id: CategoryId,

    /// Unique identifier
    pub hash: Hash,

    /// Whether category is deleted.
    pub deleted: bool,

    /// Whether category is archived.
    pub archived: bool,

    /// Number of subcategories (deleted, archived or neither),
    /// unmoderated threads and moderated threads, _directly_ in this category.
    ///
    /// As noted, the first is unaffected by any change in state of direct subcategory.
    ///
    /// The sum of the latter two only increases, and former is incremented
    /// for each new thread added to this category. A new thread is added
    /// with a `nr_in_category` equal to this sum.
    ///
    /// When there is a moderation
    /// of a thread, the variables are incremented and decremented, respectively.
    ///
    /// These values are vital for light clients, in order to validate that they are
    /// not being censored from subcategories or threads in a category.
    pub num_direct_subcategories: u32,
    pub num_direct_unmoderated_threads: u32,
    pub num_direct_moderated_threads: u32,

    /// Position as child in parent, if present, otherwise this category is a root category
    pub position_in_parent_category: Option<ChildPositionInParentCategory<CategoryId>>,

    /// Sticky threads list
    pub sticky_thread_ids: Vec<ThreadId>,
}

/// Implement total thread calcuation for category
impl<CategoryId, ThreadId, Hash> Category<CategoryId, ThreadId, Hash> {
    /// How many threads created both moderated and unmoderated
    pub fn num_threads_created(&self) -> u32 {
        self.num_direct_unmoderated_threads + self.num_direct_moderated_threads
    }
}

/// Represents a sequence of categories which have child-parent relatioonship
/// where last element is final ancestor, or root, in the context of the category tree.
type CategoryTreePath<CategoryId, ThreadId, Hash> = Vec<Category<CategoryId, ThreadId, Hash>>;

// TODO: remove when this issue is solved https://github.com/rust-lang/rust-clippy/issues/3381
// temporary type for functions argument
type CategoryTreePathArg<CategoryId, ThreadId, Hash> = [Category<CategoryId, ThreadId, Hash>];

decl_storage! {
    trait Store for Module<T: Trait> as Forum_1_1 {
        /// Map category identifier to corresponding category.
        pub CategoryById get(category_by_id) config(): map T::CategoryId => Category<T::CategoryId, T::ThreadId, T::Hash>;

        /// Category identifier value to be used for the next Category created.
        pub NextCategoryId get(next_category_id) config(): T::CategoryId;

        /// Map thread identifier to corresponding thread.
        pub ThreadById get(thread_by_id) config(): map T::ThreadId => Thread<T::ForumUserId, T::ModeratorId, T::CategoryId, T::Moment, T::Hash>;

        /// Thread identifier value to be used for next Thread in threadById.
        pub NextThreadId get(next_thread_id) config(): T::ThreadId;

        /// Map post identifier to corresponding post.
        pub PostById get(post_by_id) config(): map T::PostId => Post<T::ForumUserId, T::ModeratorId, T::ThreadId, T::Hash>;

        /// Post identifier value to be used for for next post created.
        pub NextPostId get(next_post_id) config(): T::PostId;

        /// Max depth of category.
        pub MaxCategoryDepth get(max_category_depth) config(): u8;

        /// Moderator set for each Category
        pub CategoryByModerator get(category_by_moderator) config(): double_map T::CategoryId, blake2_256(T::ModeratorId) => bool;

        /// Each account 's reaction to a post.
        pub ReactionByPost get(reaction_by_post) config(): double_map T::PostId, blake2_256(T::ForumUserId) => PostReaction;

        /// Input constraints for description text of each item in poll.
        pub PollDescConstraint get(poll_desc_constraint) config(): InputValidationLengthConstraint;

        /// Input constraints for number of items in poll.
        pub PollItemsConstraint get(poll_items_constraint) config(): InputValidationLengthConstraint;

        /// Input constraints for user name.
        pub UserNameConstraint get(user_name_constraint) config(): InputValidationLengthConstraint;

        /// Input constraints for user introduction.
        pub UserSelfIntroductionConstraint get(user_self_introduction_constraint) config(): InputValidationLengthConstraint;

        /// Input constraints for post footer.
        pub PostFooterConstraint get(post_footer_constraint) config(): InputValidationLengthConstraint;

        /// If data migration is done, set as configible for unit test purpose
        pub DataMigrationDone get(data_migration_done) config(): bool;
    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as Trait>::CategoryId,
        <T as Trait>::ThreadId,
        <T as Trait>::PostId,
        <T as Trait>::ForumUserId,
    {
        /// A category was introduced
        CategoryCreated(CategoryId),

        /// A category with given id was updated.
        /// The second argument reflects the new archival status of the category, if changed.
        /// The third argument reflects the new deletion status of the category, if changed.
        CategoryUpdated(CategoryId, Option<bool>, Option<bool>),

        /// A thread with given id was created.
        ThreadCreated(ThreadId),

        /// A thread with given id was moderated.
        ThreadModerated(ThreadId),

        /// Post with given id was created.
        PostAdded(PostId),

        /// Post with givne id was moderated.
        PostModerated(PostId),

        /// Post with given id had its text updated.
        /// The second argument reflects the number of total edits when the text update occurs.
        PostTextUpdated(PostId, u64),

        /// Thumb up post
        PostReacted(ForumUserId, PostId, PostReaction),

        /// Vote on poll
        VoteOnPoll(ThreadId, u32),

        /// Max category depth updated
        MaxCategoryDepthUpdated(u8),

        /// Sticky thread updated for category
        CategoryStickyThreadUpdate(CategoryId, Vec<ThreadId>),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Enable a moderator can moderate a category and its sub categories.
        fn set_moderator_category(origin, moderator_id: T::ModeratorId, category_id: T::CategoryId, new_value: bool) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;
            clear_prefix(b"Forum ForumUserById");

            let who = ensure_signed(origin)?;

            // Not signed by forum LEAD
            Self::ensure_is_forum_lead(&who)?;

            // ensure category exists.
            ensure!(
                <CategoryById<T>>::exists(&category_id),
                ERROR_CATEGORY_DOES_NOT_EXIST
            );

            // Get moderator id.
            Self::ensure_is_moderator_with_correct_account(&who, &moderator_id)?;

            // Put moderator into category by moderator map
            <CategoryByModerator<T>>::mutate(category_id, moderator_id, |value|
                *value = new_value);

            Ok(())
        }

        /// Set max category depth.
        fn set_max_category_depth(origin, max_category_depth: u8) -> dispatch::Result {
            let who = ensure_signed(origin)?;

            // Not signed by forum LEAD
            Self::ensure_is_forum_lead(&who)?;

            // Store new value into runtime
            MaxCategoryDepth::mutate(|value| *value = max_category_depth );

            // Store event into runtime
            Self::deposit_event(RawEvent::MaxCategoryDepthUpdated(max_category_depth));

            Ok(())
        }

        /// Add a new category.
        fn create_category(origin, parent: Option<T::CategoryId>, hash: T::Hash) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Not signed by forum LEAD
            Self::ensure_is_forum_lead(&who)?;

            // Set a temporal mutable variable
            let mut position_in_parent_category_field = None;

            // If not root, then check that we can create in parent category
            if let Some(parent_category_id) = parent {

                // Get the path from parent category to root
                let category_tree_path = Self::ensure_valid_category_and_build_category_tree_path(parent_category_id)?;

                // Check if max depth reached
                if category_tree_path.len() >= MaxCategoryDepth::get() as usize {
                    return Err(ERROR_EXCEED_MAX_CATEGORY_DEPTH);
                }

                // Can we mutate in this category?
                Self::ensure_can_add_subcategory_path_leaf(&category_tree_path)?;

                // Increment number of subcategories to reflect this new category being
                // added as a child
                <CategoryById<T>>::mutate(parent_category_id, |c| {
                    c.num_direct_subcategories += 1;
                });

                // Set `position_in_parent_category_field`
                let parent_category = category_tree_path.first().unwrap();

                // Update the variable with real data
                position_in_parent_category_field = Some(ChildPositionInParentCategory{
                    parent_id: parent_category_id,
                    child_nr_in_parent_category: parent_category.num_direct_subcategories
                });
            }

            // Get next category id
            let next_category_id = <NextCategoryId<T>>::get();

            // Create new category
            let new_category = Category {
                id : next_category_id,
                hash,
                deleted: false,
                archived: false,
                num_direct_subcategories: 0,
                num_direct_unmoderated_threads: 0,
                num_direct_moderated_threads: 0,
                position_in_parent_category: position_in_parent_category_field,
                sticky_thread_ids: vec![],
            };

            // Insert category in map
            <CategoryById<T>>::mutate(next_category_id, |value| *value = new_category);

            // Update other next category id
            <NextCategoryId<T>>::mutate(|value| *value += One::one());

            // Generate event
            Self::deposit_event(RawEvent::CategoryCreated(next_category_id));

            Ok(())
        }

        /// Update category
        fn update_category(origin, category_id: T::CategoryId, new_archival_status: Option<bool>, new_deletion_status: Option<bool>) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Not signed by forum LEAD
            Self::ensure_is_forum_lead(&who)?;

            // Make sure something is actually being changed
            ensure!(
                new_archival_status.is_some() || new_deletion_status.is_some(),
                ERROR_CATEGORY_NOT_BEING_UPDATED
            );

            // Make sure category existed.
            ensure!(
                <CategoryById<T>>::exists(&category_id),
                ERROR_CATEGORY_DOES_NOT_EXIST
                );

            // Get parent category
            let parent_category = <CategoryById<T>>::get(&category_id).position_in_parent_category;

            if let Some(unwrapped_parent_category) = parent_category {
                // Get path from parent to root of category tree.
                let category_tree_path = Self::ensure_valid_category_and_build_category_tree_path(unwrapped_parent_category.parent_id)?;

                if Self::ensure_can_mutate_in_path_leaf(&category_tree_path).is_err() {
                    // if ancestor archived or deleted, no necessary to set child again.
                    if new_archival_status == Some(true) || new_deletion_status == Some(true) {
                        return Ok(())
                    }
                };
            };

            // Get the category
            let category = <CategoryById<T>>::get(category_id);

            // Can not unarchive if category already deleted
            ensure!(
                !category.deleted || (new_deletion_status == Some(false)),
                ERROR_CATEGORY_CANNOT_BE_UNARCHIVED_WHEN_DELETED
            );

            // no any change then return Ok, no update and no event.
            let deletion_unchanged = new_deletion_status == None || new_deletion_status == Some(category.deleted);
            let archive_unchanged = new_archival_status == None || new_archival_status == Some(category.archived);

            // No any change, invalid transaction
            if deletion_unchanged && archive_unchanged {
                return Err(ERROR_CATEGORY_NOT_BEING_UPDATED)
            }

            // Mutate category, and set possible new change parameters
            <CategoryById<T>>::mutate(category_id, |c| {

                if let Some(archived) = new_archival_status {
                    c.archived = archived;
                }

                if let Some(deleted) = new_deletion_status {
                    c.deleted = deleted;
                }
            });

            // Generate event
            Self::deposit_event(RawEvent::CategoryUpdated(category_id, new_archival_status, new_deletion_status));

            Ok(())
        }

        /// Create new thread in category with poll
        fn create_thread(origin, forum_user_id: T::ForumUserId, category_id: T::CategoryId, hash: T::Hash,
            poll: Option<Poll<T::Moment, T::Hash>>,
        ) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Check that account is forum member
            Self::ensure_is_forum_member_with_correct_account(&who, &forum_user_id)?;

            // Keep next thread id
            let next_thread_id = <NextThreadId<T>>::get();

            // Create a new thread
            Self::add_new_thread(category_id, forum_user_id, hash, &poll)?;

            // Generate event
            Self::deposit_event(RawEvent::ThreadCreated(next_thread_id));

            Ok(())
        }

        /// submit a poll
        fn vote_on_poll(origin, forum_user_id: T::ForumUserId, thread_id: T::ThreadId, index: u32) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // get forum user id.
            Self::ensure_is_forum_member_with_correct_account(&who, &forum_user_id)?;

            // Get thread
            let thread = Self::ensure_thread_exists(&thread_id)?;

            // Make sure poll exist
            Self::ensure_vote_is_valid(&thread, index)?;

            // Store new poll alternative statistics
            let poll = thread.poll.unwrap();
            let new_poll_alternatives: Vec<PollAlternative<T::Hash>> = poll.poll_alternatives
                .iter()
                .enumerate()
                .map(|(old_index, old_value)| if index as usize == old_index
                    { PollAlternative {
                        hash: old_value.hash,
                        vote_count: old_value.vote_count + 1,
                    }
                    } else {
                        old_value.clone()
                    })
                .collect();

            // Update thread with one object
            <ThreadById<T>>::mutate(thread_id, |value| {
                *value = Thread {
                    poll: Some( Poll {
                        poll_alternatives: new_poll_alternatives,
                        ..poll
                    }),
                    ..(value.clone())
                }
            });

            // Store the event
            Self::deposit_event(RawEvent::VoteOnPoll(thread_id, index));

            Ok(())
        }

        /// Moderate thread
        fn moderate_thread(origin, moderator_id: T::ModeratorId, thread_id: T::ThreadId) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Ensure origin is medorator
            Self::ensure_is_moderator_with_correct_account(&who, &moderator_id)?;

            // Get thread
            let mut thread = Self::ensure_thread_exists(&thread_id)?;

            // Thread is not already moderated
            ensure!(thread.moderation.is_none(), ERROR_THREAD_ALREADY_MODERATED);

            // ensure origin can moderate category
            Self::ensure_moderate_category(&who, &moderator_id, thread.category_id)?;

            // Can mutate in corresponding category
            let path = Self::build_category_tree_path(thread.category_id);

            // Path must be non-empty, as category id is from thread in state
            assert!(!path.is_empty());

            // Path can be updated
            Self::ensure_can_mutate_in_path_leaf(&path)?;

            // Add moderation to thread
            thread.moderation = Some(ModerationAction {
                moderator_id,
            });

            // Insert new value into map
            <ThreadById<T>>::mutate(thread_id, |value| *value = thread.clone());

            // Update moderation/umoderation count of corresponding category
            <CategoryById<T>>::mutate(thread.category_id, |category| {
                category.num_direct_unmoderated_threads -= 1;
                category.num_direct_moderated_threads += 1;
            });

            // Generate event
            Self::deposit_event(RawEvent::ThreadModerated(thread_id));

            Ok(())
        }

        /// Edit post text
        fn add_post(origin, forum_user_id: T::ForumUserId, thread_id: T::ThreadId, hash: T::Hash) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            /*
             * Update SPEC with new errors,
             */

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Check that account is forum member
            Self::ensure_is_forum_member_with_correct_account(&who, &forum_user_id)?;

            // Keep next post id
            let next_post_id = <NextPostId<T>>::get();

            // Add new post
            Self::add_new_post(thread_id, hash, forum_user_id)?;

            // Generate event
            Self::deposit_event(RawEvent::PostAdded(next_post_id));

            Ok(())
        }

        /// like or unlike a post.
        fn react_post(origin, forum_user_id: T::ForumUserId, post_id: T::PostId, react: PostReaction) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Check that account is forum member
            Self::ensure_is_forum_member_with_correct_account(&who, &forum_user_id)?;

            // Make sure there exists a mutable post with post id `post_id`
            let _ = Self::ensure_post_is_mutable(&post_id)?;

            // If react is meaningful
            if react == PostReaction::NonReacton {
                return Ok(())
            }

            // Get old value in map
            let old_value = <ReactionByPost::<T>>::get(post_id, forum_user_id);

            // Update and save event.
            if old_value != react {
                <ReactionByPost::<T>>::mutate(post_id, forum_user_id, |value| *value = react);
                Self::deposit_event(RawEvent::PostReacted(forum_user_id, post_id, react));
            }

            Ok(())
        }

        /// Moderate post
        fn moderate_post(origin, moderator_id: T::ModeratorId, post_id: T::PostId) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Get moderator id.
            Self::ensure_is_moderator_with_correct_account(&who, &moderator_id)?;

            // Make sure post exists and is mutable
            let post = Self::ensure_post_is_mutable(&post_id)?;

            // make sure origin can moderate the category
            let thread = Self::ensure_thread_exists(&post.thread_id)?;

            // ensure the moderator can moderate the category
            Self::ensure_moderate_category(&who, &moderator_id, thread.category_id)?;

            // Update moderation action on post
            let moderation_action = ModerationAction{
                moderator_id,
            };

            // Update post with moderation
            <PostById<T>>::mutate(post_id, |p| p.moderation = Some(moderation_action));

            // Update moderated and unmoderated post count of corresponding thread
            <ThreadById<T>>::mutate(post.thread_id, |t| {
                t.num_unmoderated_posts -= 1;
                t.num_moderated_posts += 1;
            });

            // Generate event
            Self::deposit_event(RawEvent::PostModerated(post_id));

            Ok(())
        }

        /// Set stickied threads for category
        fn  set_stickied_threads(origin, moderator_id: T::ModeratorId, category_id: T::CategoryId, stickied_ids: Vec<T::ThreadId>) -> dispatch::Result {
            // Ensure data migration is done
            Self::ensure_data_migration_done()?;

            // Check that its a valid signature
            let who = ensure_signed(origin)?;

            // Get moderator id.
            Self::ensure_is_moderator_with_correct_account(&who, &moderator_id)?;

            // ensure the moderator can moderate the category
            Self::ensure_moderate_category(&who, &moderator_id, category_id)?;

            // Ensure all thread id valid and is under the category
            for item in &stickied_ids {
                Self::ensure_thread_belongs_to_category(*item, category_id)?;
            }

            // Update category
            <CategoryById<T>>::mutate(category_id, |category| category.sticky_thread_ids = stickied_ids.clone());

            // Generate event
            Self::deposit_event(RawEvent::CategoryStickyThreadUpdate(category_id, stickied_ids));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // TODO need a safer approach for system call
    // Interface to add a new thread.
    // It can be call from other module and this module.
    // Method not check the forum user. The extrinsic call it should check if forum id is valid.
    // If other module call it, could set the forum user id as zero, which not used by forum module.
    // Data structure of poll data: item description vector, poll description, start time, end time,
    // minimum selected items, maximum selected items
    pub fn add_new_thread(
        category_id: T::CategoryId,
        author_id: T::ForumUserId,
        hash: T::Hash,
        poll: &Option<Poll<T::Moment, T::Hash>>,
    ) -> Result<
        Thread<T::ForumUserId, T::ModeratorId, T::CategoryId, T::Moment, T::Hash>,
        &'static str,
    > {
        // Ensure data migration is done
        Self::ensure_data_migration_done()?;

        // Get path from parent to root of category tree.
        let category_tree_path =
            Self::ensure_valid_category_and_build_category_tree_path(category_id)?;

        // No ancestor is blocking us doing mutation in this category
        Self::ensure_can_mutate_in_path_leaf(&category_tree_path)?;

        // Unwrap poll
        if let Some(data) = poll {
            // Check all poll alternatives
            Self::ensure_poll_alternatives_valid(&data.poll_alternatives)?;

            // Check poll self information
            Self::ensure_poll_is_valid(&data)?;
        }

        // Get the category
        let category = <CategoryById<T>>::get(category_id);

        // Create and add new thread
        let new_thread_id = <NextThreadId<T>>::get();

        // Add inital post to thread
        let _ = Self::add_new_post(new_thread_id, hash, author_id);

        // Build a new thread
        let new_thread = Thread {
            category_id,
            hash,
            moderation: None,
            author_id,
            poll: poll.clone(),
            nr_in_category: category.num_threads_created() + 1,
            num_unmoderated_posts: 0,
            num_moderated_posts: 0,
        };

        // Store thread
        <ThreadById<T>>::mutate(new_thread_id, |value| *value = new_thread.clone());

        // Update next thread id
        <NextThreadId<T>>::mutate(|n| *n += One::one());

        // Update unmoderated thread count in corresponding category
        <CategoryById<T>>::mutate(category_id, |c| c.num_direct_unmoderated_threads += 1);

        Ok(new_thread)
    }

    // TODO need a safer approach for system call
    // Interface to add a new post.
    // It can be call from other module and this module.
    // Method not check the forum user. The extrinsic call it should check if forum id is valid.
    // If other module call it, could set the forum user id as zero, which not used by forum module.
    pub fn add_new_post(
        thread_id: T::ThreadId,
        hash: T::Hash,
        author_id: T::ForumUserId,
    ) -> Result<Post<T::ForumUserId, T::ModeratorId, T::ThreadId, T::Hash>, &'static str> {
        // Ensure data migration is done
        Self::ensure_data_migration_done()?;

        // Make sure thread exists and is mutable
        let thread = Self::ensure_thread_is_mutable(&thread_id)?;

        // Get path from parent to root of category tree.
        let category_tree_path =
            Self::ensure_valid_category_and_build_category_tree_path(thread.category_id)?;

        // No ancestor is blocking us doing mutation in this category
        Self::ensure_can_mutate_in_path_leaf(&category_tree_path)?;

        // Make and add initial post
        let new_post_id = <NextPostId<T>>::get();

        // Build a post
        let new_post = Post {
            thread_id,
            hash,
            moderation: None,
            author_id,
            nr_in_thread: thread.num_posts_ever_created(),
        };

        // Store post
        <PostById<T>>::mutate(new_post_id, |value| *value = new_post.clone());

        // Update next post id
        <NextPostId<T>>::mutate(|n| *n += One::one());

        // Update unmoderated post count of thread
        <ThreadById<T>>::mutate(thread_id, |t| t.num_unmoderated_posts += 1);

        Ok(new_post)
    }

    // Ensure poll is valid
    fn ensure_poll_is_valid(poll: &Poll<T::Moment, T::Hash>) -> dispatch::Result {
        // Poll end time must larger than now
        if poll.end_time < <timestamp::Module<T>>::now() {
            return Err(ERROR_POLL_TIME_SETTING);
        }
        // Check the timestamp setting
        if poll.start_time > poll.end_time {
            return Err(ERROR_POLL_TIME_SETTING);
        }

        Ok(())
    }

    // Ensure all poll alternative valid
    fn ensure_poll_alternatives_valid(
        alternatives: &[PollAlternative<T::Hash>],
    ) -> dispatch::Result {
        let len = alternatives.len();
        // Check alternative amount
        Self::ensure_poll_alternatives_length_is_valid(len)?;

        Ok(())
    }

    // Ensure poll alternative size is valid
    fn ensure_poll_alternatives_length_is_valid(len: usize) -> dispatch::Result {
        PollItemsConstraint::get().ensure_valid(
            len,
            ERROR_POLL_ALTERNATIVES_TOO_SHORT,
            ERROR_POLL_ALTERNATIVES_TOO_LONG,
        )
    }

    fn ensure_post_is_mutable(
        post_id: &T::PostId,
    ) -> Result<Post<T::ForumUserId, T::ModeratorId, T::ThreadId, T::Hash>, &'static str> {
        // Make sure post exists
        let post = Self::ensure_post_exists(post_id)?;

        // and is unmoderated
        ensure!(post.moderation.is_none(), ERROR_POST_MODERATED);

        // and make sure thread is mutable
        Self::ensure_thread_is_mutable(&post.thread_id)?;

        Ok(post)
    }

    fn ensure_post_exists(
        post_id: &T::PostId,
    ) -> Result<Post<T::ForumUserId, T::ModeratorId, T::ThreadId, T::Hash>, &'static str> {
        if <PostById<T>>::exists(post_id) {
            Ok(<PostById<T>>::get(post_id))
        } else {
            Err(ERROR_POST_DOES_NOT_EXIST)
        }
    }

    fn ensure_thread_is_mutable(
        thread_id: &T::ThreadId,
    ) -> Result<
        Thread<T::ForumUserId, T::ModeratorId, T::CategoryId, T::Moment, T::Hash>,
        &'static str,
    > {
        // Make sure thread exists
        let thread = Self::ensure_thread_exists(&thread_id)?;

        // and is unmoderated
        ensure!(thread.moderation.is_none(), ERROR_THREAD_MODERATED);

        // and corresponding category is mutable
        Self::ensure_catgory_is_mutable(thread.category_id)?;

        Ok(thread)
    }

    fn ensure_thread_exists(
        thread_id: &T::ThreadId,
    ) -> Result<
        Thread<T::ForumUserId, T::ModeratorId, T::CategoryId, T::Moment, T::Hash>,
        &'static str,
    > {
        if <ThreadById<T>>::exists(thread_id) {
            Ok(<ThreadById<T>>::get(thread_id))
        } else {
            Err(ERROR_THREAD_DOES_NOT_EXIST)
        }
    }

    fn ensure_is_forum_lead(account_id: &T::AccountId) -> dispatch::Result {
        let is_lead = T::is_lead(account_id);

        ensure!(is_lead, ERROR_ORIGIN_NOT_FORUM_LEAD);
        Ok(())
    }

    /// Ensure forum user id registered and its account id matched
    fn ensure_is_forum_member_with_correct_account(
        account_id: &T::AccountId,
        forum_user_id: &T::ForumUserId,
    ) -> dispatch::Result {
        let is_member = T::is_forum_member(account_id, forum_user_id);

        ensure!(is_member, ERROR_FORUM_USER_ID_NOT_MATCH_ACCOUNT);
        Ok(())
    }

    /// Ensure moderator id registered and its accound id matched
    fn ensure_is_moderator_with_correct_account(
        account_id: &T::AccountId,
        moderator_id: &T::ModeratorId,
    ) -> dispatch::Result {
        let is_moderator = T::is_moderator(account_id, moderator_id);

        ensure!(is_moderator, ERROR_MODERATOR_ID_NOT_MATCH_ACCOUNT);
        Ok(())
    }

    fn ensure_catgory_is_mutable(category_id: T::CategoryId) -> dispatch::Result {
        let category_tree_path = Self::build_category_tree_path(category_id);

        Self::ensure_can_mutate_in_path_leaf(&category_tree_path)
    }

    fn ensure_can_mutate_in_path_leaf(
        category_tree_path: &CategoryTreePathArg<T::CategoryId, T::ThreadId, T::Hash>,
    ) -> dispatch::Result {
        // Is parent category directly or indirectly deleted or archived category
        ensure!(
            !category_tree_path
                .iter()
                .any(|c: &Category<T::CategoryId, T::ThreadId, T::Hash>| c.deleted || c.archived),
            ERROR_ANCESTOR_CATEGORY_IMMUTABLE
        );

        Ok(())
    }

    fn ensure_can_add_subcategory_path_leaf(
        category_tree_path: &CategoryTreePathArg<T::CategoryId, T::ThreadId, T::Hash>,
    ) -> dispatch::Result {
        Self::ensure_can_mutate_in_path_leaf(category_tree_path)?;

        // Does adding a new category exceed maximum depth
        let depth_of_new_category = 1 + 1 + category_tree_path.len();

        ensure!(
            depth_of_new_category <= MaxCategoryDepth::get() as usize,
            ERROR_MAX_VALID_CATEGORY_DEPTH_EXCEEDED
        );

        Ok(())
    }

    /// Build category tree path and validate them
    fn ensure_valid_category_and_build_category_tree_path(
        category_id: T::CategoryId,
    ) -> Result<CategoryTreePath<T::CategoryId, T::ThreadId, T::Hash>, &'static str> {
        ensure!(
            <CategoryById<T>>::exists(&category_id),
            ERROR_CATEGORY_DOES_NOT_EXIST
        );

        // Get path from parent to root of category tree.
        let category_tree_path = Self::build_category_tree_path(category_id);

        assert!(!category_tree_path.len() > 0);

        Ok(category_tree_path)
    }

    /// Builds path and populates in `path`.
    /// Requires that `category_id` is valid
    fn build_category_tree_path(
        category_id: T::CategoryId,
    ) -> CategoryTreePath<T::CategoryId, T::ThreadId, T::Hash> {
        // Get path from parent to root of category tree.
        let mut category_tree_path = vec![];

        Self::_build_category_tree_path(category_id, &mut category_tree_path);

        category_tree_path
    }

    /// Builds path and populates in `path`.
    /// Requires that `category_id` is valid
    fn _build_category_tree_path(
        category_id: T::CategoryId,
        path: &mut CategoryTreePath<T::CategoryId, T::ThreadId, T::Hash>,
    ) {
        // Grab category
        let category = <CategoryById<T>>::get(category_id);

        // Add category to path container
        path.push(category.clone());

        // Make recursive call on parent if we are not at root
        if let Some(parent) = category.position_in_parent_category {
            assert!(<CategoryById<T>>::exists(parent.parent_id));

            Self::_build_category_tree_path(parent.parent_id, path);
        }
    }

    /// check if an account can moderate a category.
    fn ensure_moderate_category(
        account_id: &T::AccountId,
        moderator_id: &T::ModeratorId,
        category_id: T::CategoryId,
    ) -> Result<(), &'static str> {
        // Get path from category to root
        let category_tree_path = Self::build_category_tree_path(category_id);

        // Ensure moderator account registered before
        Self::ensure_is_moderator_with_correct_account(account_id, moderator_id)?;

        // Iterate path, check all ancient category
        for item in category_tree_path {
            if <CategoryByModerator<T>>::get(item.id, moderator_id) {
                return Ok(());
            }
        }
        Err(ERROR_MODERATOR_MODERATE_CATEGORY)
    }

    /// Check the vote is valid
    fn ensure_vote_is_valid(
        thread: &Thread<T::ForumUserId, T::ModeratorId, T::CategoryId, T::Moment, T::Hash>,
        index: u32,
    ) -> Result<(), &'static str> {
        // Poll not existed
        if thread.poll.is_none() {
            return Err(ERROR_POLL_NOT_EXIST);
        }

        let poll = thread.poll.as_ref().unwrap();
        // Poll not expired
        if poll.end_time < <timestamp::Module<T>>::now() {
            Err(ERROR_POLL_COMMIT_EXPIRED)
        } else {
            let alternative_length = poll.poll_alternatives.len();
            // The selected alternative index is valid
            if index as usize >= alternative_length {
                Err(ERROR_POLL_DATA)
            } else {
                Ok(())
            }
        }
    }

    /// Check the thread and category exists and thread in the category
    fn ensure_thread_belongs_to_category(
        thread_id: T::ThreadId,
        category_id: T::CategoryId,
    ) -> Result<(), &'static str> {
        // Ensure thread exists
        Self::ensure_thread_exists(&thread_id)?;

        // ensure category exists.
        ensure!(
            <CategoryById<T>>::exists(category_id),
            ERROR_CATEGORY_DOES_NOT_EXIST
        );

        // Ensure thread belongs to the category
        if <ThreadById<T>>::get(thread_id).category_id == category_id {
            Ok(())
        } else {
            Err(ERROR_THREAD_WITH_WRONG_CATEGORY_ID)
        }
    }

    /// Ensure data migration is done
    fn ensure_data_migration_done() -> Result<(), &'static str> {
        if DataMigrationDone::get() {
            Ok(())
        } else {
            Err(ERROR_DATA_MIGRATION_NOT_DONE)
        }
    }
}
