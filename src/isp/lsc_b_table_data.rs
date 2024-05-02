#[doc = "Register `LSC_B_TABLE_DATA` reader"]
pub type R = crate::R<LscBTableDataSpec>;
#[doc = "Register `LSC_B_TABLE_DATA` writer"]
pub type W = crate::W<LscBTableDataSpec>;
#[doc = "Field `b_sample_0` reader - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type BSample0R = crate::FieldReader<u16>;
#[doc = "Field `b_sample_0` writer - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type BSample0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `b_sample_1` reader - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type BSample1R = crate::FieldReader<u16>;
#[doc = "Field `b_sample_1` writer - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type BSample1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    pub fn b_sample_0(&self) -> BSample0R {
        BSample0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    pub fn b_sample_1(&self) -> BSample1R {
        BSample1R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    #[must_use]
    pub fn b_sample_0(&mut self) -> BSample0W<LscBTableDataSpec> {
        BSample0W::new(self, 0)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    #[must_use]
    pub fn b_sample_1(&mut self) -> BSample1W<LscBTableDataSpec> {
        BSample1W::new(self, 12)
    }
}
#[doc = "Sample table blue\n\nNote: The programmed sample value is immediately written into the RAM. The \n\n\n\nRAM address is generated per auto-increment. The parameter RAMs for Lens Shade \n\n\n\nCorrection and Bad Pixel Correction can only be programmed, if the RGB Bayer path is \n\n\n\nswitched on via ISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_b_table_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_b_table_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscBTableDataSpec;
impl crate::RegisterSpec for LscBTableDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_b_table_data::R`](R) reader structure"]
impl crate::Readable for LscBTableDataSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_b_table_data::W`](W) writer structure"]
impl crate::Writable for LscBTableDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_B_TABLE_DATA to value 0"]
impl crate::Resettable for LscBTableDataSpec {
    const RESET_VALUE: u32 = 0;
}
