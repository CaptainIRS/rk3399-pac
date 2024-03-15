#[doc = "Register `ROOT_ERROR_COMMAND` reader"]
pub type R = crate::R<RootErrorCommandSpec>;
#[doc = "Register `ROOT_ERROR_COMMAND` writer"]
pub type W = crate::W<RootErrorCommandSpec>;
#[doc = "Field `CERE` reader - Correctable Error Reporting Enable \\[CERE\\]
If this bit is set, the core will active its CORRECTABLE_ERROR_OUT output in response to an error message received from the link."]
pub type CereR = crate::BitReader;
#[doc = "Field `CERE` writer - Correctable Error Reporting Enable \\[CERE\\]
If this bit is set, the core will active its CORRECTABLE_ERROR_OUT output in response to an error message received from the link."]
pub type CereW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFERE` reader - Non-Fatal Error Reporting Enable \\[NFERE\\]
If this bit is set, the core will active its NON_FATAL_ERROR_OUT output in response to an error message received from the link."]
pub type NfereR = crate::BitReader;
#[doc = "Field `NFERE` writer - Non-Fatal Error Reporting Enable \\[NFERE\\]
If this bit is set, the core will active its NON_FATAL_ERROR_OUT output in response to an error message received from the link."]
pub type NfereW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERE` reader - Fatal Error Reporting Enable \\[FERE\\]
If this bit is set, the core will active its FATAL_ERROR_OUT output in response to an error message received from the link."]
pub type FereR = crate::BitReader;
#[doc = "Field `FERE` writer - Fatal Error Reporting Enable \\[FERE\\]
If this bit is set, the core will active its FATAL_ERROR_OUT output in response to an error message received from the link."]
pub type FereW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R44` reader - Reserved \\[R44\\]
Reserved"]
pub type R44R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Correctable Error Reporting Enable \\[CERE\\]
If this bit is set, the core will active its CORRECTABLE_ERROR_OUT output in response to an error message received from the link."]
    #[inline(always)]
    pub fn cere(&self) -> CereR {
        CereR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Fatal Error Reporting Enable \\[NFERE\\]
If this bit is set, the core will active its NON_FATAL_ERROR_OUT output in response to an error message received from the link."]
    #[inline(always)]
    pub fn nfere(&self) -> NfereR {
        NfereR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Error Reporting Enable \\[FERE\\]
If this bit is set, the core will active its FATAL_ERROR_OUT output in response to an error message received from the link."]
    #[inline(always)]
    pub fn fere(&self) -> FereR {
        FereR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Reserved \\[R44\\]
Reserved"]
    #[inline(always)]
    pub fn r44(&self) -> R44R {
        R44R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Correctable Error Reporting Enable \\[CERE\\]
If this bit is set, the core will active its CORRECTABLE_ERROR_OUT output in response to an error message received from the link."]
    #[inline(always)]
    #[must_use]
    pub fn cere(&mut self) -> CereW<RootErrorCommandSpec> {
        CereW::new(self, 0)
    }
    #[doc = "Bit 1 - Non-Fatal Error Reporting Enable \\[NFERE\\]
If this bit is set, the core will active its NON_FATAL_ERROR_OUT output in response to an error message received from the link."]
    #[inline(always)]
    #[must_use]
    pub fn nfere(&mut self) -> NfereW<RootErrorCommandSpec> {
        NfereW::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Error Reporting Enable \\[FERE\\]
If this bit is set, the core will active its FATAL_ERROR_OUT output in response to an error message received from the link."]
    #[inline(always)]
    #[must_use]
    pub fn fere(&mut self) -> FereW<RootErrorCommandSpec> {
        FereW::new(self, 2)
    }
}
#[doc = "Root Error Command Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_error_command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_error_command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootErrorCommandSpec;
impl crate::RegisterSpec for RootErrorCommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_error_command::R`](R) reader structure"]
impl crate::Readable for RootErrorCommandSpec {}
#[doc = "`write(|w| ..)` method takes [`root_error_command::W`](W) writer structure"]
impl crate::Writable for RootErrorCommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROOT_ERROR_COMMAND to value 0"]
impl crate::Resettable for RootErrorCommandSpec {
    const RESET_VALUE: u32 = 0;
}
