#[doc = "Register `EMMCCORE_RESP0` reader"]
pub type R = crate::R<EmmccoreResp0Spec>;
#[doc = "Register `EMMCCORE_RESP0` writer"]
pub type W = crate::W<EmmccoreResp0Spec>;
#[doc = "Field `RESP` reader - Response register bit \\[31:0\\]"]
pub type RespR = crate::FieldReader<u32>;
#[doc = "Field `RESP` writer - Response register bit \\[31:0\\]"]
pub type RespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Response register bit \\[31:0\\]"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Response register bit \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<EmmccoreResp0Spec> {
        RespW::new(self, 0)
    }
}
#[doc = "Response register bit \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_resp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_resp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreResp0Spec;
impl crate::RegisterSpec for EmmccoreResp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_resp0::R`](R) reader structure"]
impl crate::Readable for EmmccoreResp0Spec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_resp0::W`](W) writer structure"]
impl crate::Writable for EmmccoreResp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_RESP0 to value 0"]
impl crate::Resettable for EmmccoreResp0Spec {
    const RESET_VALUE: u32 = 0;
}
