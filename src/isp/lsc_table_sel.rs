#[doc = "Register `LSC_TABLE_SEL` reader"]
pub type R = crate::R<LscTableSelSpec>;
#[doc = "Register `LSC_TABLE_SEL` writer"]
pub type W = crate::W<LscTableSelSpec>;
#[doc = "Field `table_sel` reader - 0: next active tables set is\n\ntable set 0. 1: next active tables\n\nset is table set 1.\n\nTable selection is object of a shadowing mechnism.\n\nThe current status is visible at"]
pub type TableSelR = crate::BitReader;
#[doc = "Field `table_sel` writer - 0: next active tables set is\n\ntable set 0. 1: next active tables\n\nset is table set 1.\n\nTable selection is object of a shadowing mechnism.\n\nThe current status is visible at"]
pub type TableSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: next active tables set is\n\ntable set 0. 1: next active tables\n\nset is table set 1.\n\nTable selection is object of a shadowing mechnism.\n\nThe current status is visible at"]
    #[inline(always)]
    pub fn table_sel(&self) -> TableSelR {
        TableSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: next active tables set is\n\ntable set 0. 1: next active tables\n\nset is table set 1.\n\nTable selection is object of a shadowing mechnism.\n\nThe current status is visible at"]
    #[inline(always)]
    #[must_use]
    pub fn table_sel(&mut self) -> TableSelW<LscTableSelSpec> {
        TableSelW::new(self, 0)
    }
}
#[doc = "Lens shade table set selection\n\nNote: Table set 0 access by SW at table address 0...153. Table set 1 access at table \n\naddress 154...307. For LSC4_MSZ the table set 1 is physically not available: \n\nISP_LSC_TABLE_SEL shall always be 0 for this HW configuration. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_table_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_table_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscTableSelSpec;
impl crate::RegisterSpec for LscTableSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_table_sel::R`](R) reader structure"]
impl crate::Readable for LscTableSelSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_table_sel::W`](W) writer structure"]
impl crate::Writable for LscTableSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_TABLE_SEL to value 0"]
impl crate::Resettable for LscTableSelSpec {
    const RESET_VALUE: u32 = 0;
}
