#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
// 功能模块宏
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    // 配置接口宏
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // 事件宏
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransferred(T::AccountId, T::AccountId, Vec<u8>),
    }

    // 错误信息宏
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
        ProofTooLong,
    }

    // 存储单元宏
    #[pallet::storage]
    pub(super) type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 不同时期区块的执行逻辑宏
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // 可调用函数宏
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // 创建凭证
        #[pallet::weight(1_000)]
        pub fn create_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResultWithPostInfo {
            let mut too_long = false;
            if proof.len() > 5 {
                too_long = true;
            }
            ensure!(
                !too_long, 
                Error::<T>::ProofTooLong
            );
            // 校验当前交易发送方是否签名，返回值为发送方ID
            let sender = ensure_signed(origin)?;
            // 校验凭证是否已经存在，存在返回一个错误
            ensure!(
                !Proofs::<T>::contains_key(&proof),
                Error::<T>::ProofAlreadyClaimed
            );
            // 获取当前的区块号
            let current_block = <frame_system::Pallet<T>>::block_number();
            // 将凭证写入
            Proofs::<T>::insert(&proof, (&sender, current_block));
            // 保存已创建事件
            Self::deposit_event(Event::ClaimCreated(sender, proof));
            Ok(().into())
        }

        // 销毁凭证
        #[pallet::weight(10_000)]
        pub fn revoke_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResultWithPostInfo {
            // 校验当前交易发送方是否签名，返回值为发送方ID
            let sender = ensure_signed(origin)?;
            // 校验凭证是否存在，不存在报错
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            // 获取凭证的所有者
            let (owner, _) = Proofs::<T>::get(&proof);
            // 校验所有者和发送方是否一至，不一致报错
            ensure!(sender == owner, Error::<T>::NotProofOwner);
            // 销毁凭证
            Proofs::<T>::remove(&proof);
            // 保存已销毁事件
            Self::deposit_event(Event::ClaimRevoked(sender, proof));
            Ok(().into())
        }

        // 转移凭证
        #[pallet::weight(10_000)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            proof: Vec<u8>,
            receiver: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // 校验当前交易发送方是否签名，返回值为发送方ID
            let sender = ensure_signed(origin)?;
            // 校验凭证是否存在，不存在报错
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            // 获取凭证的所有者
            let (owner, _) = Proofs::<T>::get(&proof);
            // 校验所有者和发送方是否一至，不一致报错
            ensure!(sender == owner, Error::<T>::NotProofOwner);
            // 获取当前的区块号
            let current_block = <frame_system::Pallet<T>>::block_number();
            // 更新凭证的所有者
            Proofs::<T>::insert(&proof, (&receiver, current_block));
            // 保存已转移事件
            Self::deposit_event(Event::ClaimTransferred(sender, receiver, proof));
            Ok(().into())
        }
    }
}
