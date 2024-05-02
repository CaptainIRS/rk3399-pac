#[doc = "Register `LSC_R_TABLE_DATA` reader"]
pub type R = crate::R<LscRTableDataSpec>;
#[doc = "Register `LSC_R_TABLE_DATA` writer"]
pub type W = crate::W<LscRTableDataSpec>;
#[doc = "Field `r_sample_0` reader - correction factor at sample point (fixed point number:\n\n2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type RSample0R = crate::FieldReader<u16>;
#[doc = "Field `r_sample_0` writer - correction factor at sample point (fixed point number:\n\n2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type RSample0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_sample_1` reader - correction factor at sample point (fixed point\n\nnumber: 2 bits integer with 10-bit fractional part,\n\nrange 1..3.999)\n\n\n\n"]
pub type RSample1R = crate::FieldReader<u16>;
#[doc = "Field `r_sample_1` writer - correction factor at sample point (fixed point\n\nnumber: 2 bits integer with 10-bit fractional part,\n\nrange 1..3.999)\n\n\n\n"]
pub type RSample1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number:\n\n2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    pub fn r_sample_0(&self) -> RSample0R {
        RSample0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point\n\nnumber: 2 bits integer with 10-bit fractional part,\n\nrange 1..3.999)\n\n\n\n"]
    #[inline(always)]
    pub fn r_sample_1(&self) -> RSample1R {
        RSample1R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number:\n\n2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    #[must_use]
    pub fn r_sample_0(&mut self) -> RSample0W<LscRTableDataSpec> {
        RSample0W::new(self, 0)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point\n\nnumber: 2 bits integer with 10-bit fractional part,\n\nrange 1..3.999)\n\n\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn r_sample_1(&mut self) -> RSample1W<LscRTableDataSpec> {
        RSample1W::new(self, 12)
    }
}
#[doc = "Sample table red\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_r_table_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_r_table_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscRTableDataSpec;
impl crate::RegisterSpec for LscRTableDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_r_table_data::R`](R) reader structure"]
impl crate::Readable for LscRTableDataSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_r_table_data::W`](W) writer structure"]
impl crate::Writable for LscRTableDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_R_TABLE_DATA to value 0"]
impl crate::Resettable for LscRTableDataSpec {
    const RESET_VALUE: u32 = 0;
}
