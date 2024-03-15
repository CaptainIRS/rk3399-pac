#[doc = "Register `RKI2C_SCL_OE_DB` reader"]
pub type R = crate::R<Rki2cSclOeDbSpec>;
#[doc = "Register `RKI2C_SCL_OE_DB` writer"]
pub type W = crate::W<Rki2cSclOeDbSpec>;
#[doc = "Field `SCL_OE_DB` reader - slave hold scl debounce cycles for debounce (unit: Tclk_i2c)"]
pub type SclOeDbR = crate::FieldReader;
#[doc = "Field `SCL_OE_DB` writer - slave hold scl debounce cycles for debounce (unit: Tclk_i2c)"]
pub type SclOeDbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - slave hold scl debounce cycles for debounce (unit: Tclk_i2c)"]
    #[inline(always)]
    pub fn scl_oe_db(&self) -> SclOeDbR {
        SclOeDbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - slave hold scl debounce cycles for debounce (unit: Tclk_i2c)"]
    #[inline(always)]
    #[must_use]
    pub fn scl_oe_db(&mut self) -> SclOeDbW<Rki2cSclOeDbSpec> {
        SclOeDbW::new(self, 0)
    }
}
#[doc = "slave hold debounce configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_scl_oe_db::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rki2c_scl_oe_db::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cSclOeDbSpec;
impl crate::RegisterSpec for Rki2cSclOeDbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_scl_oe_db::R`](R) reader structure"]
impl crate::Readable for Rki2cSclOeDbSpec {}
#[doc = "`write(|w| ..)` method takes [`rki2c_scl_oe_db::W`](W) writer structure"]
impl crate::Writable for Rki2cSclOeDbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RKI2C_SCL_OE_DB to value 0x20"]
impl crate::Resettable for Rki2cSclOeDbSpec {
    const RESET_VALUE: u32 = 0x20;
}
