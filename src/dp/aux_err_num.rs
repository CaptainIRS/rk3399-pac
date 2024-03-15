#[doc = "Register `AUX_ERR_NUM` reader"]
pub type R = crate::R<AuxErrNumSpec>;
#[doc = "Register `AUX_ERR_NUM` writer"]
pub type W = crate::W<AuxErrNumSpec>;
#[doc = "Field `AUX_ERR_NUM` reader - The error number counter of AUX channel counts when AUX channel access failed. In AUX CH reading, this number indicates the number of read back byte. In AUX CH writing, this number indicates the number of reply command."]
pub type AuxErrNumR = crate::FieldReader;
#[doc = "Field `AUX_ERR_NUM` writer - The error number counter of AUX channel counts when AUX channel access failed. In AUX CH reading, this number indicates the number of read back byte. In AUX CH writing, this number indicates the number of reply command."]
pub type AuxErrNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The error number counter of AUX channel counts when AUX channel access failed. In AUX CH reading, this number indicates the number of read back byte. In AUX CH writing, this number indicates the number of reply command."]
    #[inline(always)]
    pub fn aux_err_num(&self) -> AuxErrNumR {
        AuxErrNumR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The error number counter of AUX channel counts when AUX channel access failed. In AUX CH reading, this number indicates the number of read back byte. In AUX CH writing, this number indicates the number of reply command."]
    #[inline(always)]
    #[must_use]
    pub fn aux_err_num(&mut self) -> AuxErrNumW<AuxErrNumSpec> {
        AuxErrNumW::new(self, 0)
    }
}
#[doc = "AUX Channel Access Error Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_err_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_err_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxErrNumSpec;
impl crate::RegisterSpec for AuxErrNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_err_num::R`](R) reader structure"]
impl crate::Readable for AuxErrNumSpec {}
#[doc = "`write(|w| ..)` method takes [`aux_err_num::W`](W) writer structure"]
impl crate::Writable for AuxErrNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUX_ERR_NUM to value 0"]
impl crate::Resettable for AuxErrNumSpec {
    const RESET_VALUE: u32 = 0;
}
