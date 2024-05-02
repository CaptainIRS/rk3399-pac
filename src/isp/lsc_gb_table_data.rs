#[doc = "Register `LSC_GB_TABLE_DATA` reader"]
pub type R = crate::R<LscGbTableDataSpec>;
#[doc = "Register `LSC_GB_TABLE_DATA` writer"]
pub type W = crate::W<LscGbTableDataSpec>;
#[doc = "Field `gb_sample_0` reader - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GbSample0R = crate::FieldReader<u16>;
#[doc = "Field `gb_sample_0` writer - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GbSample0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `gb_sample_1` reader - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GbSample1R = crate::FieldReader<u16>;
#[doc = "Field `gb_sample_1` writer - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GbSample1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    pub fn gb_sample_0(&self) -> GbSample0R {
        GbSample0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    pub fn gb_sample_1(&self) -> GbSample1R {
        GbSample1R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    #[must_use]
    pub fn gb_sample_0(&mut self) -> GbSample0W<LscGbTableDataSpec> {
        GbSample0W::new(self, 0)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    #[must_use]
    pub fn gb_sample_1(&mut self) -> GbSample1W<LscGbTableDataSpec> {
        GbSample1W::new(self, 12)
    }
}
#[doc = "Sample table green (blue)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits).Table set 0 access by SW at table address 0...153. Table \n\nset 1 access at table address 154...307. \n\n\n\n \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_gb_table_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_gb_table_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscGbTableDataSpec;
impl crate::RegisterSpec for LscGbTableDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_gb_table_data::R`](R) reader structure"]
impl crate::Readable for LscGbTableDataSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_gb_table_data::W`](W) writer structure"]
impl crate::Writable for LscGbTableDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_GB_TABLE_DATA to value 0"]
impl crate::Resettable for LscGbTableDataSpec {
    const RESET_VALUE: u32 = 0;
}
