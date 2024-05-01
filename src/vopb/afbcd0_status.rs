#[doc = "Register `AFBCD0_STATUS` reader"]
pub type R = crate::R<Afbcd0StatusSpec>;
#[doc = "Register `AFBCD0_STATUS` writer"]
pub type W = crate::W<Afbcd0StatusSpec>;
#[doc = "Field `AFBCD_HREG_IDLE_N` reader - afbcd_hreg_idle_n"]
pub type AfbcdHregIdleNR = crate::BitReader;
#[doc = "Field `AFBCD_HREG_IDLE_N` writer - afbcd_hreg_idle_n"]
pub type AfbcdHregIdleNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFBCD_HREG_DEC_RESP` reader - afbcd_hreg_dec_resp"]
pub type AfbcdHregDecRespR = crate::BitReader;
#[doc = "Field `AFBCD_HREG_DEC_RESP` writer - afbcd_hreg_dec_resp"]
pub type AfbcdHregDecRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFBCD_HREG_AXI_RRESP` reader - afbcd_hreg_axi_rresp"]
pub type AfbcdHregAxiRrespR = crate::BitReader;
#[doc = "Field `AFBCD_HREG_AXI_RRESP` writer - afbcd_hreg_axi_rresp"]
pub type AfbcdHregAxiRrespW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - afbcd_hreg_idle_n"]
    #[inline(always)]
    pub fn afbcd_hreg_idle_n(&self) -> AfbcdHregIdleNR {
        AfbcdHregIdleNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - afbcd_hreg_dec_resp"]
    #[inline(always)]
    pub fn afbcd_hreg_dec_resp(&self) -> AfbcdHregDecRespR {
        AfbcdHregDecRespR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - afbcd_hreg_axi_rresp"]
    #[inline(always)]
    pub fn afbcd_hreg_axi_rresp(&self) -> AfbcdHregAxiRrespR {
        AfbcdHregAxiRrespR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - afbcd_hreg_idle_n"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_idle_n(&mut self) -> AfbcdHregIdleNW<Afbcd0StatusSpec> {
        AfbcdHregIdleNW::new(self, 0)
    }
    #[doc = "Bit 1 - afbcd_hreg_dec_resp"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_dec_resp(&mut self) -> AfbcdHregDecRespW<Afbcd0StatusSpec> {
        AfbcdHregDecRespW::new(self, 1)
    }
    #[doc = "Bit 2 - afbcd_hreg_axi_rresp"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_axi_rresp(&mut self) -> AfbcdHregAxiRrespW<Afbcd0StatusSpec> {
        AfbcdHregAxiRrespW::new(self, 2)
    }
}
#[doc = "AFBCD0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afbcd0StatusSpec;
impl crate::RegisterSpec for Afbcd0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afbcd0_status::R`](R) reader structure"]
impl crate::Readable for Afbcd0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`afbcd0_status::W`](W) writer structure"]
impl crate::Writable for Afbcd0StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFBCD0_STATUS to value 0"]
impl crate::Resettable for Afbcd0StatusSpec {
    const RESET_VALUE: u32 = 0;
}
