#[doc = "Register `CRU_CLKGATE_CON17` reader"]
pub type R = crate::R<CruClkgateCon17Spec>;
#[doc = "Register `CRU_CLKGATE_CON17` writer"]
pub type W = crate::W<CruClkgateCon17Spec>;
#[doc = "Field `ACLK_VCODEC_EN` reader - aclk_vcodec clock disable bit When HIGH, disable clock"]
pub type AclkVcodecEnR = crate::BitReader;
#[doc = "Field `ACLK_VCODEC_EN` writer - aclk_vcodec clock disable bit When HIGH, disable clock"]
pub type AclkVcodecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VCODEC_NOC_EN` reader - aclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkVcodecNocEnR = crate::BitReader;
#[doc = "Field `ACLK_VCODEC_NOC_EN` writer - aclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkVcodecNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VCODEC_EN` reader - hclk_vcodec clock disable bit When HIGH, disable clock"]
pub type HclkVcodecEnR = crate::BitReader;
#[doc = "Field `HCLK_VCODEC_EN` writer - hclk_vcodec clock disable bit When HIGH, disable clock"]
pub type HclkVcodecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VCODEC_NOC_EN` reader - hclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkVcodecNocEnR = crate::BitReader;
#[doc = "Field `HCLK_VCODEC_NOC_EN` writer - hclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkVcodecNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VDU_EN` reader - aclk_vdu clock disable bit When HIGH, disable clock"]
pub type AclkVduEnR = crate::BitReader;
#[doc = "Field `ACLK_VDU_EN` writer - aclk_vdu clock disable bit When HIGH, disable clock"]
pub type AclkVduEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VDU_NOC_EN` reader - aclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkVduNocEnR = crate::BitReader;
#[doc = "Field `ACLK_VDU_NOC_EN` writer - aclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkVduNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VDU_EN` reader - hclk_vdu clock disable bit When HIGH, disable clock"]
pub type HclkVduEnR = crate::BitReader;
#[doc = "Field `HCLK_VDU_EN` writer - hclk_vdu clock disable bit When HIGH, disable clock"]
pub type HclkVduEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VDU_NOC_EN` reader - hclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkVduNocEnR = crate::BitReader;
#[doc = "Field `HCLK_VDU_NOC_EN` writer - hclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkVduNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_vcodec clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vcodec_en(&self) -> AclkVcodecEnR {
        AclkVcodecEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_vcodec_noc_en(&self) -> AclkVcodecNocEnR {
        AclkVcodecNocEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hclk_vcodec clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vcodec_en(&self) -> HclkVcodecEnR {
        HclkVcodecEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_vcodec_noc_en(&self) -> HclkVcodecNocEnR {
        HclkVcodecNocEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_vdu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vdu_en(&self) -> AclkVduEnR {
        AclkVduEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_vdu_noc_en(&self) -> AclkVduNocEnR {
        AclkVduNocEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hclk_vdu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vdu_en(&self) -> HclkVduEnR {
        HclkVduEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_vdu_noc_en(&self) -> HclkVduNocEnR {
        HclkVduNocEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_vcodec clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vcodec_en(&mut self) -> AclkVcodecEnW<CruClkgateCon17Spec> {
        AclkVcodecEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vcodec_noc_en(&mut self) -> AclkVcodecNocEnW<CruClkgateCon17Spec> {
        AclkVcodecNocEnW::new(self, 1)
    }
    #[doc = "Bit 2 - hclk_vcodec clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vcodec_en(&mut self) -> HclkVcodecEnW<CruClkgateCon17Spec> {
        HclkVcodecEnW::new(self, 2)
    }
    #[doc = "Bit 3 - hclk_vcodec_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vcodec_noc_en(&mut self) -> HclkVcodecNocEnW<CruClkgateCon17Spec> {
        HclkVcodecNocEnW::new(self, 3)
    }
    #[doc = "Bit 8 - aclk_vdu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vdu_en(&mut self) -> AclkVduEnW<CruClkgateCon17Spec> {
        AclkVduEnW::new(self, 8)
    }
    #[doc = "Bit 9 - aclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vdu_noc_en(&mut self) -> AclkVduNocEnW<CruClkgateCon17Spec> {
        AclkVduNocEnW::new(self, 9)
    }
    #[doc = "Bit 10 - hclk_vdu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vdu_en(&mut self) -> HclkVduEnW<CruClkgateCon17Spec> {
        HclkVduEnW::new(self, 10)
    }
    #[doc = "Bit 11 - hclk_vdu_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vdu_noc_en(&mut self) -> HclkVduNocEnW<CruClkgateCon17Spec> {
        HclkVduNocEnW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon17Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon17Spec;
impl crate::RegisterSpec for CruClkgateCon17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con17::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon17Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con17::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON17 to value 0"]
impl crate::Resettable for CruClkgateCon17Spec {
    const RESET_VALUE: u32 = 0;
}
