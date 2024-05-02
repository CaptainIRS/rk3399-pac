#[doc = "Register `CAC_A` reader"]
pub type R = crate::R<CacASpec>;
#[doc = "Register `CAC_A` writer"]
pub type W = crate::W<CacASpec>;
#[doc = "Field `A_Red` reader - Parameter A_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
pub type ARedR = crate::FieldReader<u16>;
#[doc = "Field `A_Red` writer - Parameter A_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
pub type ARedW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `A_Blue` reader - Parameter A_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
pub type ABlueR = crate::FieldReader<u16>;
#[doc = "Field `A_Blue` writer - Parameter A_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
pub type ABlueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Parameter A_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
    #[inline(always)]
    pub fn a_red(&self) -> ARedR {
        ARedR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Parameter A_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
    #[inline(always)]
    pub fn a_blue(&self) -> ABlueR {
        ABlueR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Parameter A_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn a_red(&mut self) -> ARedW<CacASpec> {
        ARedW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Parameter A_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
    #[inline(always)]
    #[must_use]
    pub fn a_blue(&mut self) -> ABlueW<CacASpec> {
        ABlueW::new(self, 16)
    }
}
#[doc = "Linear Parameters for radial shift calculation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacASpec;
impl crate::RegisterSpec for CacASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cac_a::R`](R) reader structure"]
impl crate::Readable for CacASpec {}
#[doc = "`write(|w| ..)` method takes [`cac_a::W`](W) writer structure"]
impl crate::Writable for CacASpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAC_A to value 0"]
impl crate::Resettable for CacASpec {
    const RESET_VALUE: u32 = 0;
}
