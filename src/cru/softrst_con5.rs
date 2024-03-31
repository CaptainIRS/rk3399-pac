#[doc = "Register `SOFTRST_CON5` reader"]
pub type R = crate::R<SoftrstCon5Spec>;
#[doc = "Register `SOFTRST_CON5` writer"]
pub type W = crate::W<SoftrstCon5Spec>;
#[doc = "Field `ARESETN_VCODEC_NOC_REQ` reader - aresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVcodecNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_VCODEC_NOC_REQ` writer - aresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVcodecNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_VCODEC_REQ` reader - aresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVcodecReqR = crate::BitReader;
#[doc = "Field `ARESETN_VCODEC_REQ` writer - aresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVcodecReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VCODEC_NOC_REQ` reader - hresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVcodecNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_VCODEC_NOC_REQ` writer - hresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVcodecNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VCODEC_REQ` reader - hresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVcodecReqR = crate::BitReader;
#[doc = "Field `HRESETN_VCODEC_REQ` writer - hresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVcodecReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_VDU_NOC_REQ` reader - aresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVduNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_VDU_NOC_REQ` writer - aresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVduNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_VDU_REQ` reader - aresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVduReqR = crate::BitReader;
#[doc = "Field `ARESETN_VDU_REQ` writer - aresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVduReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VDU_NOC_REQ` reader - hresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVduNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_VDU_NOC_REQ` writer - hresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVduNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_VDU_REQ` reader - hresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVduReqR = crate::BitReader;
#[doc = "Field `HRESETN_VDU_REQ` writer - hresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnVduReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_VDU_CORE_REQ` reader - resetn_vdu_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVduCoreReqR = crate::BitReader;
#[doc = "Field `RESETN_VDU_CORE_REQ` writer - resetn_vdu_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVduCoreReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_VDU_CA_REQ` reader - resetn_vdu_ca request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVduCaReqR = crate::BitReader;
#[doc = "Field `RESETN_VDU_CA_REQ` writer - resetn_vdu_ca request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnVduCaReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vcodec_noc_req(&self) -> AresetnVcodecNocReqR {
        AresetnVcodecNocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vcodec_req(&self) -> AresetnVcodecReqR {
        AresetnVcodecReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vcodec_noc_req(&self) -> HresetnVcodecNocReqR {
        HresetnVcodecNocReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vcodec_req(&self) -> HresetnVcodecReqR {
        HresetnVcodecReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - aresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vdu_noc_req(&self) -> AresetnVduNocReqR {
        AresetnVduNocReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vdu_req(&self) -> AresetnVduReqR {
        AresetnVduReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vdu_noc_req(&self) -> HresetnVduNocReqR {
        HresetnVduNocReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_vdu_req(&self) -> HresetnVduReqR {
        HresetnVduReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_vdu_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_vdu_core_req(&self) -> ResetnVduCoreReqR {
        ResetnVduCoreReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_vdu_ca request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_vdu_ca_req(&self) -> ResetnVduCaReqR {
        ResetnVduCaReqR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vcodec_noc_req(&mut self) -> AresetnVcodecNocReqW<SoftrstCon5Spec> {
        AresetnVcodecNocReqW::new(self, 0)
    }
    #[doc = "Bit 1 - aresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vcodec_req(&mut self) -> AresetnVcodecReqW<SoftrstCon5Spec> {
        AresetnVcodecReqW::new(self, 1)
    }
    #[doc = "Bit 2 - hresetn_vcodec_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vcodec_noc_req(&mut self) -> HresetnVcodecNocReqW<SoftrstCon5Spec> {
        HresetnVcodecNocReqW::new(self, 2)
    }
    #[doc = "Bit 3 - hresetn_vcodec request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vcodec_req(&mut self) -> HresetnVcodecReqW<SoftrstCon5Spec> {
        HresetnVcodecReqW::new(self, 3)
    }
    #[doc = "Bit 8 - aresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vdu_noc_req(&mut self) -> AresetnVduNocReqW<SoftrstCon5Spec> {
        AresetnVduNocReqW::new(self, 8)
    }
    #[doc = "Bit 9 - aresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vdu_req(&mut self) -> AresetnVduReqW<SoftrstCon5Spec> {
        AresetnVduReqW::new(self, 9)
    }
    #[doc = "Bit 10 - hresetn_vdu_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vdu_noc_req(&mut self) -> HresetnVduNocReqW<SoftrstCon5Spec> {
        HresetnVduNocReqW::new(self, 10)
    }
    #[doc = "Bit 11 - hresetn_vdu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_vdu_req(&mut self) -> HresetnVduReqW<SoftrstCon5Spec> {
        HresetnVduReqW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_vdu_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_vdu_core_req(&mut self) -> ResetnVduCoreReqW<SoftrstCon5Spec> {
        ResetnVduCoreReqW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_vdu_ca request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_vdu_ca_req(&mut self) -> ResetnVduCaReqW<SoftrstCon5Spec> {
        ResetnVduCaReqW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon5Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon5Spec;
impl crate::RegisterSpec for SoftrstCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con5::R`](R) reader structure"]
impl crate::Readable for SoftrstCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con5::W`](W) writer structure"]
impl crate::Writable for SoftrstCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON5 to value 0"]
impl crate::Resettable for SoftrstCon5Spec {
    const RESET_VALUE: u32 = 0;
}
