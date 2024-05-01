#[doc = "Register `MMU_CTRL0` reader"]
pub type R = crate::R<MmuCtrl0Spec>;
#[doc = "Register `MMU_CTRL0` writer"]
pub type W = crate::W<MmuCtrl0Spec>;
#[doc = "RGA MMU Page table size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwMmuPageSize {
    #[doc = "0: 4KB page"]
    B0 = 0,
    #[doc = "1: 64KB page"]
    B1 = 1,
}
impl From<SwMmuPageSize> for bool {
    #[inline(always)]
    fn from(variant: SwMmuPageSize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_MMU_PAGE_SIZE` reader - RGA MMU Page table size"]
pub type SwMmuPageSizeR = crate::BitReader<SwMmuPageSize>;
impl SwMmuPageSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwMmuPageSize {
        match self.bits {
            false => SwMmuPageSize::B0,
            true => SwMmuPageSize::B1,
        }
    }
    #[doc = "4KB page"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwMmuPageSize::B0
    }
    #[doc = "64KB page"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwMmuPageSize::B1
    }
}
#[doc = "Field `SW_MMU_PAGE_SIZE` writer - RGA MMU Page table size"]
pub type SwMmuPageSizeW<'a, REG> = crate::BitWriter<'a, REG, SwMmuPageSize>;
impl<'a, REG> SwMmuPageSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4KB page"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwMmuPageSize::B0)
    }
    #[doc = "64KB page"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwMmuPageSize::B1)
    }
}
#[doc = "RGA CMD channel MMU enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCmdMmuEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwCmdMmuEn> for bool {
    #[inline(always)]
    fn from(variant: SwCmdMmuEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CMD_MMU_EN` reader - RGA CMD channel MMU enable"]
pub type SwCmdMmuEnR = crate::BitReader<SwCmdMmuEn>;
impl SwCmdMmuEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwCmdMmuEn {
        match self.bits {
            false => SwCmdMmuEn::B0,
            true => SwCmdMmuEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwCmdMmuEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwCmdMmuEn::B1
    }
}
#[doc = "Field `SW_CMD_MMU_EN` writer - RGA CMD channel MMU enable"]
pub type SwCmdMmuEnW<'a, REG> = crate::BitWriter<'a, REG, SwCmdMmuEn>;
impl<'a, REG> SwCmdMmuEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCmdMmuEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCmdMmuEn::B1)
    }
}
#[doc = "Field `SW_CMD_MMU_FLUSH` reader - RGA CMD channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwCmdMmuFlushR = crate::BitReader;
#[doc = "Field `SW_CMD_MMU_FLUSH` writer - RGA CMD channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
pub type SwCmdMmuFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SRC_CH_PRIORITY` reader - sw_src_ch_priority"]
pub type SwSrcChPriorityR = crate::FieldReader;
#[doc = "Field `SW_SRC_CH_PRIORITY` writer - sw_src_ch_priority"]
pub type SwSrcChPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_SRC1_CH_PRIORITY` reader - sw_src1_ch_priority"]
pub type SwSrc1ChPriorityR = crate::FieldReader;
#[doc = "Field `SW_SRC1_CH_PRIORITY` writer - sw_src1_ch_priority"]
pub type SwSrc1ChPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_DST_CH_PRIORITY` reader - sw_dst_ch_priority"]
pub type SwDstChPriorityR = crate::FieldReader;
#[doc = "Field `SW_DST_CH_PRIORITY` writer - sw_dst_ch_priority"]
pub type SwDstChPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_ELS_CH_PRIORITY` reader - sw_els_ch_priority"]
pub type SwElsChPriorityR = crate::FieldReader;
#[doc = "Field `SW_ELS_CH_PRIORITY` writer - sw_els_ch_priority"]
pub type SwElsChPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - RGA MMU Page table size"]
    #[inline(always)]
    pub fn sw_mmu_page_size(&self) -> SwMmuPageSizeR {
        SwMmuPageSizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RGA CMD channel MMU enable"]
    #[inline(always)]
    pub fn sw_cmd_mmu_en(&self) -> SwCmdMmuEnR {
        SwCmdMmuEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RGA CMD channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    pub fn sw_cmd_mmu_flush(&self) -> SwCmdMmuFlushR {
        SwCmdMmuFlushR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - sw_src_ch_priority"]
    #[inline(always)]
    pub fn sw_src_ch_priority(&self) -> SwSrcChPriorityR {
        SwSrcChPriorityR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - sw_src1_ch_priority"]
    #[inline(always)]
    pub fn sw_src1_ch_priority(&self) -> SwSrc1ChPriorityR {
        SwSrc1ChPriorityR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - sw_dst_ch_priority"]
    #[inline(always)]
    pub fn sw_dst_ch_priority(&self) -> SwDstChPriorityR {
        SwDstChPriorityR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - sw_els_ch_priority"]
    #[inline(always)]
    pub fn sw_els_ch_priority(&self) -> SwElsChPriorityR {
        SwElsChPriorityR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RGA MMU Page table size"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mmu_page_size(&mut self) -> SwMmuPageSizeW<MmuCtrl0Spec> {
        SwMmuPageSizeW::new(self, 0)
    }
    #[doc = "Bit 1 - RGA CMD channel MMU enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_mmu_en(&mut self) -> SwCmdMmuEnW<MmuCtrl0Spec> {
        SwCmdMmuEnW::new(self, 1)
    }
    #[doc = "Bit 2 - RGA CMD channel MMU TLB flush:\n\nSet 1 to this bit to flush MMU TLB, auto clear"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_mmu_flush(&mut self) -> SwCmdMmuFlushW<MmuCtrl0Spec> {
        SwCmdMmuFlushW::new(self, 2)
    }
    #[doc = "Bits 3:4 - sw_src_ch_priority"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_ch_priority(&mut self) -> SwSrcChPriorityW<MmuCtrl0Spec> {
        SwSrcChPriorityW::new(self, 3)
    }
    #[doc = "Bits 5:6 - sw_src1_ch_priority"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_ch_priority(&mut self) -> SwSrc1ChPriorityW<MmuCtrl0Spec> {
        SwSrc1ChPriorityW::new(self, 5)
    }
    #[doc = "Bits 7:8 - sw_dst_ch_priority"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_ch_priority(&mut self) -> SwDstChPriorityW<MmuCtrl0Spec> {
        SwDstChPriorityW::new(self, 7)
    }
    #[doc = "Bits 9:10 - sw_els_ch_priority"]
    #[inline(always)]
    #[must_use]
    pub fn sw_els_ch_priority(&mut self) -> SwElsChPriorityW<MmuCtrl0Spec> {
        SwElsChPriorityW::new(self, 9)
    }
}
#[doc = "RGA MMU control 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuCtrl0Spec;
impl crate::RegisterSpec for MmuCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_ctrl0::R`](R) reader structure"]
impl crate::Readable for MmuCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mmu_ctrl0::W`](W) writer structure"]
impl crate::Writable for MmuCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMU_CTRL0 to value 0"]
impl crate::Resettable for MmuCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
