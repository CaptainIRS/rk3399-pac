#[doc = "Register `MMU_STATUS` reader"]
pub type R = crate::R<MmuStatusSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MmuPagingEnabled {
    #[doc = "0: paging is disabled"]
    B0 = 0,
    #[doc = "1: Paging is enabled"]
    B1 = 1,
}
impl From<MmuPagingEnabled> for bool {
    #[inline(always)]
    fn from(variant: MmuPagingEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMU_PAGING_ENABLED` reader - "]
pub type MmuPagingEnabledR = crate::BitReader<MmuPagingEnabled>;
impl MmuPagingEnabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MmuPagingEnabled {
        match self.bits {
            false => MmuPagingEnabled::B0,
            true => MmuPagingEnabled::B1,
        }
    }
    #[doc = "paging is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MmuPagingEnabled::B0
    }
    #[doc = "Paging is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MmuPagingEnabled::B1
    }
}
#[doc = "Field `MMU_PAGE_FAULT_ACTIVE` reader - MMU page fault mode currently enabled.The\n\nmode is enabled by command\n\n1: page fault is active"]
pub type MmuPageFaultActiveR = crate::BitReader;
#[doc = "Field `MMU_STALL_ACTIVE` reader - MMU stall mode currently enabled. The mode\n\nis enabled by command.\n\n1: MMU is in stall active status"]
pub type MmuStallActiveR = crate::BitReader;
#[doc = "Field `MMU_IDLE` reader - the MMu is idle when accesses are being\n\ntranslated and there are no unfinished\n\ntranslated access. The MMU_IDLE signal only\n\nreports idle when the MMU processor is idle\n\nand accesses are active on the external bus.\n\nNote: the MMU can be idle in page fault mode.\n\n1: MMU is idle"]
pub type MmuIdleR = crate::BitReader;
#[doc = "Field `MMU_REPLAY_BUFFER_EMPTY` reader - 1:The MMU replay buffer is empty."]
pub type MmuReplayBufferEmptyR = crate::BitReader;
#[doc = "The direction of access for last page fault:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MmuPageFaultIsWrite {
    #[doc = "0: read"]
    B0 = 0,
    #[doc = "1: write"]
    B1 = 1,
}
impl From<MmuPageFaultIsWrite> for bool {
    #[inline(always)]
    fn from(variant: MmuPageFaultIsWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMU_PAGE_FAULT_IS_WRITE` reader - The direction of access for last page fault:"]
pub type MmuPageFaultIsWriteR = crate::BitReader<MmuPageFaultIsWrite>;
impl MmuPageFaultIsWriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MmuPageFaultIsWrite {
        match self.bits {
            false => MmuPageFaultIsWrite::B0,
            true => MmuPageFaultIsWrite::B1,
        }
    }
    #[doc = "read"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MmuPageFaultIsWrite::B0
    }
    #[doc = "write"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MmuPageFaultIsWrite::B1
    }
}
#[doc = "Field `MMU_PAGE_FAULT_BUS_ID` reader - Index of master responsible for the last page\n\nfault"]
pub type MmuPageFaultBusIdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mmu_paging_enabled(&self) -> MmuPagingEnabledR {
        MmuPagingEnabledR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMU page fault mode currently enabled.The\n\nmode is enabled by command\n\n1: page fault is active"]
    #[inline(always)]
    pub fn mmu_page_fault_active(&self) -> MmuPageFaultActiveR {
        MmuPageFaultActiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMU stall mode currently enabled. The mode\n\nis enabled by command.\n\n1: MMU is in stall active status"]
    #[inline(always)]
    pub fn mmu_stall_active(&self) -> MmuStallActiveR {
        MmuStallActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the MMu is idle when accesses are being\n\ntranslated and there are no unfinished\n\ntranslated access. The MMU_IDLE signal only\n\nreports idle when the MMU processor is idle\n\nand accesses are active on the external bus.\n\nNote: the MMU can be idle in page fault mode.\n\n1: MMU is idle"]
    #[inline(always)]
    pub fn mmu_idle(&self) -> MmuIdleR {
        MmuIdleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:The MMU replay buffer is empty."]
    #[inline(always)]
    pub fn mmu_replay_buffer_empty(&self) -> MmuReplayBufferEmptyR {
        MmuReplayBufferEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The direction of access for last page fault:"]
    #[inline(always)]
    pub fn mmu_page_fault_is_write(&self) -> MmuPageFaultIsWriteR {
        MmuPageFaultIsWriteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - Index of master responsible for the last page\n\nfault"]
    #[inline(always)]
    pub fn mmu_page_fault_bus_id(&self) -> MmuPageFaultBusIdR {
        MmuPageFaultBusIdR::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
#[doc = "MMU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuStatusSpec;
impl crate::RegisterSpec for MmuStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_status::R`](R) reader structure"]
impl crate::Readable for MmuStatusSpec {}
#[doc = "`reset()` method sets MMU_STATUS to value 0x18"]
impl crate::Resettable for MmuStatusSpec {
    const RESET_VALUE: u32 = 0x18;
}
