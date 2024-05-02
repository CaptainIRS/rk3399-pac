#[doc = "Register `CAC_C` reader"]
pub type R = crate::R<CacCSpec>;
#[doc = "Register `CAC_C` writer"]
pub type W = crate::W<CacCSpec>;
#[doc = "Field `C_Red` reader - Parameter C_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
pub type CRedR = crate::FieldReader<u16>;
#[doc = "Field `C_Red` writer - Parameter C_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
pub type CRedW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `C_Blue` reader - Parameter C_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
pub type CBlueR = crate::FieldReader<u16>;
#[doc = "Field `C_Blue` writer - Parameter C_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
pub type CBlueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Parameter C_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
    #[inline(always)]
    pub fn c_red(&self) -> CRedR {
        CRedR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Parameter C_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
    #[inline(always)]
    pub fn c_blue(&self) -> CBlueR {
        CBlueR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Parameter C_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn c_red(&mut self) -> CRedW<CacCSpec> {
        CRedW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Parameter C_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
    #[inline(always)]
    #[must_use]
    pub fn c_blue(&mut self) -> CBlueW<CacCSpec> {
        CBlueW::new(self, 16)
    }
}
#[doc = "Cubical Parameters for radial shift calculation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacCSpec;
impl crate::RegisterSpec for CacCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cac_c::R`](R) reader structure"]
impl crate::Readable for CacCSpec {}
#[doc = "`write(|w| ..)` method takes [`cac_c::W`](W) writer structure"]
impl crate::Writable for CacCSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAC_C to value 0"]
impl crate::Resettable for CacCSpec {
    const RESET_VALUE: u32 = 0;
}
