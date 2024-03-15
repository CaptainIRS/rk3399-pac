#[doc = "Register `FC_DATAUTO3` reader"]
pub type R = crate::R<FcDatauto3Spec>;
#[doc = "Register `FC_DATAUTO3` writer"]
pub type W = crate::W<FcDatauto3Spec>;
#[doc = "Field `ACR_AUTO` reader - Enables ACR packet insertion"]
pub type AcrAutoR = crate::BitReader;
#[doc = "Field `ACR_AUTO` writer - Enables ACR packet insertion"]
pub type AcrAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDI_AUTO` reader - Enables AUDI packet insertion"]
pub type AudiAutoR = crate::BitReader;
#[doc = "Field `AUDI_AUTO` writer - Enables AUDI packet insertion"]
pub type AudiAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCP_AUTO` reader - Enables GCP packet insertion"]
pub type GcpAutoR = crate::BitReader;
#[doc = "Field `GCP_AUTO` writer - Enables GCP packet insertion"]
pub type GcpAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVI_AUTO` reader - Enables AVI packet insertion"]
pub type AviAutoR = crate::BitReader;
#[doc = "Field `AVI_AUTO` writer - Enables AVI packet insertion"]
pub type AviAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_AUTO` reader - Enables AMP packet insertion"]
pub type AmpAutoR = crate::BitReader;
#[doc = "Field `AMP_AUTO` writer - Enables AMP packet insertion"]
pub type AmpAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVBI_AUTO` reader - Enables NTSC VBI packet insertion"]
pub type NvbiAutoR = crate::BitReader;
#[doc = "Field `NVBI_AUTO` writer - Enables NTSC VBI packet insertion"]
pub type NvbiAutoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables ACR packet insertion"]
    #[inline(always)]
    pub fn acr_auto(&self) -> AcrAutoR {
        AcrAutoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables AUDI packet insertion"]
    #[inline(always)]
    pub fn audi_auto(&self) -> AudiAutoR {
        AudiAutoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables GCP packet insertion"]
    #[inline(always)]
    pub fn gcp_auto(&self) -> GcpAutoR {
        GcpAutoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables AVI packet insertion"]
    #[inline(always)]
    pub fn avi_auto(&self) -> AviAutoR {
        AviAutoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables AMP packet insertion"]
    #[inline(always)]
    pub fn amp_auto(&self) -> AmpAutoR {
        AmpAutoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables NTSC VBI packet insertion"]
    #[inline(always)]
    pub fn nvbi_auto(&self) -> NvbiAutoR {
        NvbiAutoR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables ACR packet insertion"]
    #[inline(always)]
    #[must_use]
    pub fn acr_auto(&mut self) -> AcrAutoW<FcDatauto3Spec> {
        AcrAutoW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables AUDI packet insertion"]
    #[inline(always)]
    #[must_use]
    pub fn audi_auto(&mut self) -> AudiAutoW<FcDatauto3Spec> {
        AudiAutoW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables GCP packet insertion"]
    #[inline(always)]
    #[must_use]
    pub fn gcp_auto(&mut self) -> GcpAutoW<FcDatauto3Spec> {
        GcpAutoW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables AVI packet insertion"]
    #[inline(always)]
    #[must_use]
    pub fn avi_auto(&mut self) -> AviAutoW<FcDatauto3Spec> {
        AviAutoW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables AMP packet insertion"]
    #[inline(always)]
    #[must_use]
    pub fn amp_auto(&mut self) -> AmpAutoW<FcDatauto3Spec> {
        AmpAutoW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables NTSC VBI packet insertion"]
    #[inline(always)]
    #[must_use]
    pub fn nvbi_auto(&mut self) -> NvbiAutoW<FcDatauto3Spec> {
        NvbiAutoW::new(self, 5)
    }
}
#[doc = "Enables ACR packet insertion\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDatauto3Spec;
impl crate::RegisterSpec for FcDatauto3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_datauto3::R`](R) reader structure"]
impl crate::Readable for FcDatauto3Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_datauto3::W`](W) writer structure"]
impl crate::Writable for FcDatauto3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DATAUTO3 to value 0x3f"]
impl crate::Resettable for FcDatauto3Spec {
    const RESET_VALUE: u8 = 0x3f;
}
