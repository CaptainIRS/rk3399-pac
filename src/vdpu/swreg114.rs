#[doc = "Register `SWREG114` reader"]
pub type R = crate::R<Swreg114Spec>;
#[doc = "Register `SWREG114` writer"]
pub type W = crate::W<Swreg114Spec>;
#[doc = "Field `H264_POCF_LEN` reader - the length of picture order count field in stream"]
pub type H264PocfLenR = crate::FieldReader;
#[doc = "Field `H264_POCF_LEN` writer - the length of picture order count field in stream"]
pub type H264PocfLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `H264_MAX_REFIDX0` reader - the maximum reference index 0\n\nit will be used in decoding inter predicted macro blocks"]
pub type H264MaxRefidx0R = crate::FieldReader;
#[doc = "Field `H264_MAX_REFIDX0` writer - the maximum reference index 0\n\nit will be used in decoding inter predicted macro blocks"]
pub type H264MaxRefidx0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_MAX_REFIDX1` reader - the maximum reference index 1\n\nit will be used in decoding inter predicted macro blocks"]
pub type H264MaxRefidx1R = crate::FieldReader;
#[doc = "Field `H264_MAX_REFIDX1` writer - the maximum reference index 1\n\nit will be used in decoding inter predicted macro blocks"]
pub type H264MaxRefidx1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `H264_PPS_ID` reader - the id for picture parameter set\n\nit identifies the slice header will have the id of picture parameter set"]
pub type H264PpsIdR = crate::FieldReader;
#[doc = "Field `H264_PPS_ID` writer - the id for picture parameter set\n\nit identifies the slice header will have the id of picture parameter set"]
pub type H264PpsIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - the length of picture order count field in stream"]
    #[inline(always)]
    pub fn h264_pocf_len(&self) -> H264PocfLenR {
        H264PocfLenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 14:18 - the maximum reference index 0\n\nit will be used in decoding inter predicted macro blocks"]
    #[inline(always)]
    pub fn h264_max_refidx0(&self) -> H264MaxRefidx0R {
        H264MaxRefidx0R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - the maximum reference index 1\n\nit will be used in decoding inter predicted macro blocks"]
    #[inline(always)]
    pub fn h264_max_refidx1(&self) -> H264MaxRefidx1R {
        H264MaxRefidx1R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - the id for picture parameter set\n\nit identifies the slice header will have the id of picture parameter set"]
    #[inline(always)]
    pub fn h264_pps_id(&self) -> H264PpsIdR {
        H264PpsIdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the length of picture order count field in stream"]
    #[inline(always)]
    #[must_use]
    pub fn h264_pocf_len(&mut self) -> H264PocfLenW<Swreg114Spec> {
        H264PocfLenW::new(self, 0)
    }
    #[doc = "Bits 14:18 - the maximum reference index 0\n\nit will be used in decoding inter predicted macro blocks"]
    #[inline(always)]
    #[must_use]
    pub fn h264_max_refidx0(&mut self) -> H264MaxRefidx0W<Swreg114Spec> {
        H264MaxRefidx0W::new(self, 14)
    }
    #[doc = "Bits 19:23 - the maximum reference index 1\n\nit will be used in decoding inter predicted macro blocks"]
    #[inline(always)]
    #[must_use]
    pub fn h264_max_refidx1(&mut self) -> H264MaxRefidx1W<Swreg114Spec> {
        H264MaxRefidx1W::new(self, 19)
    }
    #[doc = "Bits 24:31 - the id for picture parameter set\n\nit identifies the slice header will have the id of picture parameter set"]
    #[inline(always)]
    #[must_use]
    pub fn h264_pps_id(&mut self) -> H264PpsIdW<Swreg114Spec> {
        H264PpsIdW::new(self, 24)
    }
}
#[doc = "maximum reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg114::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg114::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg114Spec;
impl crate::RegisterSpec for Swreg114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg114::R`](R) reader structure"]
impl crate::Readable for Swreg114Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg114::W`](W) writer structure"]
impl crate::Writable for Swreg114Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG114 to value 0"]
impl crate::Resettable for Swreg114Spec {
    const RESET_VALUE: u32 = 0;
}
