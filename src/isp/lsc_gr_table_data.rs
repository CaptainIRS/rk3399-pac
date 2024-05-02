#[doc = "Register `LSC_GR_TABLE_DATA` reader"]
pub type R = crate::R<LscGrTableDataSpec>;
#[doc = "Register `LSC_GR_TABLE_DATA` writer"]
pub type W = crate::W<LscGrTableDataSpec>;
#[doc = "Field `gr_sample_0` reader - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GrSample0R = crate::FieldReader<u16>;
#[doc = "Field `gr_sample_0` writer - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GrSample0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `gr_sample_1` reader - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GrSample1R = crate::FieldReader<u16>;
#[doc = "Field `gr_sample_1` writer - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
pub type GrSample1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    pub fn gr_sample_0(&self) -> GrSample0R {
        GrSample0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    pub fn gr_sample_1(&self) -> GrSample1R {
        GrSample1R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    #[must_use]
    pub fn gr_sample_0(&mut self) -> GrSample0W<LscGrTableDataSpec> {
        GrSample0W::new(self, 0)
    }
    #[doc = "Bits 12:23 - correction factor at sample point (fixed point number - 2 bits integer with 10-bit fractional part, range 1..3.999)"]
    #[inline(always)]
    #[must_use]
    pub fn gr_sample_1(&mut self) -> GrSample1W<LscGrTableDataSpec> {
        GrSample1W::new(self, 12)
    }
}
#[doc = "Sample table green (red)\n\nNote: The programmed sample value is immediately written into the RAM. The RAM \n\naddress is generated per auto-increment. The parameter RAMs for Lens Shade Correction and \n\nBad Pixel Correction can only be programmed, if the RGB Bayer path is switched on via \n\nISP_CTRL register (ISP_MODE bits). \n\n\n\nTable set 0 access by SW at table address 0...153. Table set 1 access at table address \n\n\n\n154...307. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_gr_table_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_gr_table_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscGrTableDataSpec;
impl crate::RegisterSpec for LscGrTableDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_gr_table_data::R`](R) reader structure"]
impl crate::Readable for LscGrTableDataSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_gr_table_data::W`](W) writer structure"]
impl crate::Writable for LscGrTableDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_GR_TABLE_DATA to value 0"]
impl crate::Resettable for LscGrTableDataSpec {
    const RESET_VALUE: u32 = 0;
}
