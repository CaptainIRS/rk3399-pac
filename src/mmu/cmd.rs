#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MmuCmd {
    #[doc = "0: MMU_ENABLE_PAGING. enable paging."]
    D0 = 0,
    #[doc = "1: MMU_DISABLE_PAGING. disable paging."]
    D1 = 1,
    #[doc = "2: MMU_ENABLE_STALL. turn on stall mode."]
    D2 = 2,
    #[doc = "3: MMU_DISABLE_STALL. turn off stall mode."]
    D3 = 3,
    #[doc = "4: MMU_ZAP_CACHE. zap the entire page table cache."]
    D4 = 4,
    #[doc = "5: MMU_PAGE_FAULT_DONE. leave page fault mode."]
    D5 = 5,
    #[doc = "6: MMU_FORCE_RESET. reset the mmu. The MMU_ENABLE_STALL command can always be issued. Other commands are ignored unless the MMU is idle or stalled."]
    D6 = 6,
}
impl From<MmuCmd> for u8 {
    #[inline(always)]
    fn from(variant: MmuCmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MmuCmd {
    type Ux = u8;
}
#[doc = "Field `MMU_CMD` reader - "]
pub type MmuCmdR = crate::FieldReader<MmuCmd>;
impl MmuCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MmuCmd> {
        match self.bits {
            0 => Some(MmuCmd::D0),
            1 => Some(MmuCmd::D1),
            2 => Some(MmuCmd::D2),
            3 => Some(MmuCmd::D3),
            4 => Some(MmuCmd::D4),
            5 => Some(MmuCmd::D5),
            6 => Some(MmuCmd::D6),
            _ => None,
        }
    }
    #[doc = "MMU_ENABLE_PAGING. enable paging."]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == MmuCmd::D0
    }
    #[doc = "MMU_DISABLE_PAGING. disable paging."]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == MmuCmd::D1
    }
    #[doc = "MMU_ENABLE_STALL. turn on stall mode."]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == MmuCmd::D2
    }
    #[doc = "MMU_DISABLE_STALL. turn off stall mode."]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == MmuCmd::D3
    }
    #[doc = "MMU_ZAP_CACHE. zap the entire page table cache."]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == MmuCmd::D4
    }
    #[doc = "MMU_PAGE_FAULT_DONE. leave page fault mode."]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == MmuCmd::D5
    }
    #[doc = "MMU_FORCE_RESET. reset the mmu. The MMU_ENABLE_STALL command can always be issued. Other commands are ignored unless the MMU is idle or stalled."]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == MmuCmd::D6
    }
}
#[doc = "Field `MMU_CMD` writer - "]
pub type MmuCmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, MmuCmd>;
impl<'a, REG> MmuCmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MMU_ENABLE_PAGING. enable paging."]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(MmuCmd::D0)
    }
    #[doc = "MMU_DISABLE_PAGING. disable paging."]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(MmuCmd::D1)
    }
    #[doc = "MMU_ENABLE_STALL. turn on stall mode."]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(MmuCmd::D2)
    }
    #[doc = "MMU_DISABLE_STALL. turn off stall mode."]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(MmuCmd::D3)
    }
    #[doc = "MMU_ZAP_CACHE. zap the entire page table cache."]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(MmuCmd::D4)
    }
    #[doc = "MMU_PAGE_FAULT_DONE. leave page fault mode."]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(MmuCmd::D5)
    }
    #[doc = "MMU_FORCE_RESET. reset the mmu. The MMU_ENABLE_STALL command can always be issued. Other commands are ignored unless the MMU is idle or stalled."]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(MmuCmd::D6)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mmu_cmd(&self) -> MmuCmdR {
        MmuCmdR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn mmu_cmd(&mut self) -> MmuCmdW<CmdSpec> {
        MmuCmdW::new(self, 0)
    }
}
#[doc = "MMU command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
