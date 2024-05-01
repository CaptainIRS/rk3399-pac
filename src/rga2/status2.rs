#[doc = "Register `STATUS2` reader"]
pub type R = crate::R<Status2Spec>;
#[doc = "Field `RPP_ERROR` reader - rpp_error"]
pub type RppErrorR = crate::BitReader;
#[doc = "Field `BUS_ERROR` reader - bus_error"]
pub type BusErrorR = crate::BitReader;
#[doc = "Field `SRCRPP_OUTBUF_RREADY` reader - dstrpp_outbuf_rready"]
pub type SrcrppOutbufRreadyR = crate::FieldReader;
#[doc = "Field `DSTRPP_OUTBUF_RREADY` reader - dstrpp_outbuf_rready"]
pub type DstrppOutbufRreadyR = crate::FieldReader;
#[doc = "Field `RPP_MKRAM_RREADY` reader - rpp_mkram_rready"]
pub type RppMkramRreadyR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - rpp_error"]
    #[inline(always)]
    pub fn rpp_error(&self) -> RppErrorR {
        RppErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - bus_error"]
    #[inline(always)]
    pub fn bus_error(&self) -> BusErrorR {
        BusErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - dstrpp_outbuf_rready"]
    #[inline(always)]
    pub fn srcrpp_outbuf_rready(&self) -> SrcrppOutbufRreadyR {
        SrcrppOutbufRreadyR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - dstrpp_outbuf_rready"]
    #[inline(always)]
    pub fn dstrpp_outbuf_rready(&self) -> DstrppOutbufRreadyR {
        DstrppOutbufRreadyR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:12 - rpp_mkram_rready"]
    #[inline(always)]
    pub fn rpp_mkram_rready(&self) -> RppMkramRreadyR {
        RppMkramRreadyR::new(((self.bits >> 11) & 3) as u8)
    }
}
#[doc = "RGA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status2Spec;
impl crate::RegisterSpec for Status2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status2::R`](R) reader structure"]
impl crate::Readable for Status2Spec {}
#[doc = "`reset()` method sets STATUS2 to value 0"]
impl crate::Resettable for Status2Spec {
    const RESET_VALUE: u32 = 0;
}
