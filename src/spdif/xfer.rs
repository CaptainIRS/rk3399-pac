#[doc = "Register `XFER` reader"]
pub type R = crate::R<XferSpec>;
#[doc = "Register `XFER` writer"]
pub type W = crate::W<XferSpec>;
#[doc = "Field `XFER` reader - Transfer Start Register\n\nTransfer Start Register"]
pub type XferR = crate::BitReader;
#[doc = "Field `XFER` writer - Transfer Start Register\n\nTransfer Start Register"]
pub type XferW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Start Register\n\nTransfer Start Register"]
    #[inline(always)]
    pub fn xfer(&self) -> XferR {
        XferR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Start Register\n\nTransfer Start Register"]
    #[inline(always)]
    #[must_use]
    pub fn xfer(&mut self) -> XferW<XferSpec> {
        XferW::new(self, 0)
    }
}
#[doc = "Transfer Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xfer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xfer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XferSpec;
impl crate::RegisterSpec for XferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xfer::R`](R) reader structure"]
impl crate::Readable for XferSpec {}
#[doc = "`write(|w| ..)` method takes [`xfer::W`](W) writer structure"]
impl crate::Writable for XferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XFER to value 0"]
impl crate::Resettable for XferSpec {
    const RESET_VALUE: u32 = 0;
}
