#[doc = "Register `CAC_B` reader"]
pub type R = crate::R<CacBSpec>;
#[doc = "Register `CAC_B` writer"]
pub type W = crate::W<CacBSpec>;
#[doc = "Field `B_Red` reader - Parameter B_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
pub type BRedR = crate::FieldReader<u16>;
#[doc = "Field `B_Red` writer - Parameter B_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
pub type BRedW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `B_Blue` reader - Parameter B_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
pub type BBlueR = crate::FieldReader<u16>;
#[doc = "Field `B_Blue` writer - Parameter B_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
pub type BBlueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Parameter B_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
    #[inline(always)]
    pub fn b_red(&self) -> BRedR {
        BRedR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Parameter B_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
    #[inline(always)]
    pub fn b_blue(&self) -> BBlueR {
        BBlueR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Parameter B_Red for radial red shift calculation,\n\naccording to (A_Red * r + B_Red * r^2 + C_Red * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn b_red(&mut self) -> BRedW<CacBSpec> {
        BRedW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Parameter B_Blue for radial blue shift calculation,\n\naccording to (A_Blue * r + B_Blue * r^2 + C_Blue * r^3).\n\nIt is a 9 bit twos complement integer with 4 fractional\n\ndigits value and value range from -16 up to 15.9375."]
    #[inline(always)]
    #[must_use]
    pub fn b_blue(&mut self) -> BBlueW<CacBSpec> {
        BBlueW::new(self, 16)
    }
}
#[doc = "Square Parameters for radial shift calculation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacBSpec;
impl crate::RegisterSpec for CacBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cac_b::R`](R) reader structure"]
impl crate::Readable for CacBSpec {}
#[doc = "`write(|w| ..)` method takes [`cac_b::W`](W) writer structure"]
impl crate::Writable for CacBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAC_B to value 0"]
impl crate::Resettable for CacBSpec {
    const RESET_VALUE: u32 = 0;
}
