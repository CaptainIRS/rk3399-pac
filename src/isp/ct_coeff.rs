#[doc = "Register `CT_COEFF%s` reader"]
pub type R = crate::R<CtCoeffSpec>;
#[doc = "Register `CT_COEFF%s` writer"]
pub type W = crate::W<CtCoeffSpec>;
#[doc = "Field `ct_coeff` reader - Coefficient n for cross talk matrix.\n\nValues are 11-bit signed fixed-point numbers with 4\n\nbit integer and 7 bit fractional part,\n\nranging from -8 (0x400) to +7.992 (0x3FF). 0 is\n\nreprsented by 0x000 and a coefficient value of 1 as 0x080."]
pub type CtCoeffR = crate::FieldReader<u16>;
#[doc = "Field `ct_coeff` writer - Coefficient n for cross talk matrix.\n\nValues are 11-bit signed fixed-point numbers with 4\n\nbit integer and 7 bit fractional part,\n\nranging from -8 (0x400) to +7.992 (0x3FF). 0 is\n\nreprsented by 0x000 and a coefficient value of 1 as 0x080."]
pub type CtCoeffW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Coefficient n for cross talk matrix.\n\nValues are 11-bit signed fixed-point numbers with 4\n\nbit integer and 7 bit fractional part,\n\nranging from -8 (0x400) to +7.992 (0x3FF). 0 is\n\nreprsented by 0x000 and a coefficient value of 1 as 0x080."]
    #[inline(always)]
    pub fn ct_coeff(&self) -> CtCoeffR {
        CtCoeffR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Coefficient n for cross talk matrix.\n\nValues are 11-bit signed fixed-point numbers with 4\n\nbit integer and 7 bit fractional part,\n\nranging from -8 (0x400) to +7.992 (0x3FF). 0 is\n\nreprsented by 0x000 and a coefficient value of 1 as 0x080."]
    #[inline(always)]
    #[must_use]
    pub fn ct_coeff(&mut self) -> CtCoeffW<CtCoeffSpec> {
        CtCoeffW::new(self, 0)
    }
}
#[doc = "cross-talk configuration register (color correction matrix) n (n=0..8)\n\nNote: Reset values generate a matrix which does not modify the pixel values. Reset \n\nvalues are: coeff_0 = 0x80, coeff_1 = 0x00, coeff_2 = 0x00, coeff_3 = 0x00, coeff_4 = 0x80, \n\n\n\ncoeff_5 = 0x00, coeff_6 = 0x00, coeff_7 = 0x00, coeff_8 = 0x80 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_coeff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_coeff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtCoeffSpec;
impl crate::RegisterSpec for CtCoeffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ct_coeff::R`](R) reader structure"]
impl crate::Readable for CtCoeffSpec {}
#[doc = "`write(|w| ..)` method takes [`ct_coeff::W`](W) writer structure"]
impl crate::Writable for CtCoeffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CT_COEFF%s to value 0"]
impl crate::Resettable for CtCoeffSpec {
    const RESET_VALUE: u32 = 0;
}
