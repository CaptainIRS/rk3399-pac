#[doc = "Register `CRU_CLKGATE_CON16` reader"]
pub type R = crate::R<CruClkgateCon16Spec>;
#[doc = "Register `CRU_CLKGATE_CON16` writer"]
pub type W = crate::W<CruClkgateCon16Spec>;
#[doc = "Field `ACLK_IEP_EN` reader - aclk_iep clock disable bit When HIGH, disable clock"]
pub type AclkIepEnR = crate::BitReader;
#[doc = "Field `ACLK_IEP_EN` writer - aclk_iep clock disable bit When HIGH, disable clock"]
pub type AclkIepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_IEP_NOC_EN` reader - aclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkIepNocEnR = crate::BitReader;
#[doc = "Field `ACLK_IEP_NOC_EN` writer - aclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkIepNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_IEP_EN` reader - hclk_iep clock disable bit When HIGH, disable clock"]
pub type HclkIepEnR = crate::BitReader;
#[doc = "Field `HCLK_IEP_EN` writer - hclk_iep clock disable bit When HIGH, disable clock"]
pub type HclkIepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_IEP_NOC_EN` reader - hclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkIepNocEnR = crate::BitReader;
#[doc = "Field `HCLK_IEP_NOC_EN` writer - hclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkIepNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_RGA_EN` reader - aclk_rga clock disable bit When HIGH, disable clock"]
pub type AclkRgaEnR = crate::BitReader;
#[doc = "Field `ACLK_RGA_EN` writer - aclk_rga clock disable bit When HIGH, disable clock"]
pub type AclkRgaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_RGA_NOC_EN` reader - aclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkRgaNocEnR = crate::BitReader;
#[doc = "Field `ACLK_RGA_NOC_EN` writer - aclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkRgaNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_RGA_EN` reader - hclk_rga clock disable bit When HIGH, disable clock"]
pub type HclkRgaEnR = crate::BitReader;
#[doc = "Field `HCLK_RGA_EN` writer - hclk_rga clock disable bit When HIGH, disable clock"]
pub type HclkRgaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_RGA_NOC_EN` reader - hclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkRgaNocEnR = crate::BitReader;
#[doc = "Field `HCLK_RGA_NOC_EN` writer - hclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkRgaNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_iep clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_iep_en(&self) -> AclkIepEnR {
        AclkIepEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_iep_noc_en(&self) -> AclkIepNocEnR {
        AclkIepNocEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hclk_iep clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_iep_en(&self) -> HclkIepEnR {
        HclkIepEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_iep_noc_en(&self) -> HclkIepNocEnR {
        HclkIepNocEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_rga clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_rga_en(&self) -> AclkRgaEnR {
        AclkRgaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_rga_noc_en(&self) -> AclkRgaNocEnR {
        AclkRgaNocEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hclk_rga clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_rga_en(&self) -> HclkRgaEnR {
        HclkRgaEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_rga_noc_en(&self) -> HclkRgaNocEnR {
        HclkRgaNocEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_iep clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_iep_en(&mut self) -> AclkIepEnW<CruClkgateCon16Spec> {
        AclkIepEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_iep_noc_en(&mut self) -> AclkIepNocEnW<CruClkgateCon16Spec> {
        AclkIepNocEnW::new(self, 1)
    }
    #[doc = "Bit 2 - hclk_iep clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_iep_en(&mut self) -> HclkIepEnW<CruClkgateCon16Spec> {
        HclkIepEnW::new(self, 2)
    }
    #[doc = "Bit 3 - hclk_iep_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_iep_noc_en(&mut self) -> HclkIepNocEnW<CruClkgateCon16Spec> {
        HclkIepNocEnW::new(self, 3)
    }
    #[doc = "Bit 8 - aclk_rga clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_rga_en(&mut self) -> AclkRgaEnW<CruClkgateCon16Spec> {
        AclkRgaEnW::new(self, 8)
    }
    #[doc = "Bit 9 - aclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_rga_noc_en(&mut self) -> AclkRgaNocEnW<CruClkgateCon16Spec> {
        AclkRgaNocEnW::new(self, 9)
    }
    #[doc = "Bit 10 - hclk_rga clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_rga_en(&mut self) -> HclkRgaEnW<CruClkgateCon16Spec> {
        HclkRgaEnW::new(self, 10)
    }
    #[doc = "Bit 11 - hclk_rga_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_rga_noc_en(&mut self) -> HclkRgaNocEnW<CruClkgateCon16Spec> {
        HclkRgaNocEnW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon16Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon16Spec;
impl crate::RegisterSpec for CruClkgateCon16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con16::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon16Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con16::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON16 to value 0"]
impl crate::Resettable for CruClkgateCon16Spec {
    const RESET_VALUE: u32 = 0;
}
