#[doc = "Register `FC_GMD_UP` writer"]
pub type W = crate::W<FcGmdUpSpec>;
#[doc = "Field `GMDUPDATEPACKET` writer - Gamut Metadata packet update"]
pub type GmdupdatepacketW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Gamut Metadata packet update"]
    #[inline(always)]
    #[must_use]
    pub fn gmdupdatepacket(&mut self) -> GmdupdatepacketW<FcGmdUpSpec> {
        GmdupdatepacketW::new(self, 0)
    }
}
#[doc = "Gamut Metadata packet update\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_up::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGmdUpSpec;
impl crate::RegisterSpec for FcGmdUpSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`fc_gmd_up::W`](W) writer structure"]
impl crate::Writable for FcGmdUpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_GMD_UP to value 0"]
impl crate::Resettable for FcGmdUpSpec {
    const RESET_VALUE: u8 = 0;
}
