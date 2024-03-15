#[doc = "Register `CRU_MISC_CON` reader"]
pub type R = crate::R<CruMiscConSpec>;
#[doc = "Register `CRU_MISC_CON` writer"]
pub type W = crate::W<CruMiscConSpec>;
#[doc = "Output clock selection for test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TestclkSel {
    #[doc = "0: clk_wifi"]
    H0 = 0,
    #[doc = "1: clk_wifi"]
    H1 = 1,
    #[doc = "2: clk_wifi"]
    H2 = 2,
    #[doc = "3: clk_wifi"]
    H3 = 3,
    #[doc = "4: clk_wifi"]
    H4 = 4,
    #[doc = "5: clk_wifi"]
    H5 = 5,
    #[doc = "6: clk_wifi"]
    H6 = 6,
    #[doc = "7: clk_wifi"]
    H7 = 7,
    #[doc = "8: clk_wifi"]
    H8 = 8,
    #[doc = "9: clk_wifi"]
    H9 = 9,
    #[doc = "10: clk_wifi"]
    Ha = 10,
    #[doc = "11: clk_wifi"]
    Hb = 11,
    #[doc = "12: clk_wifi"]
    Hc = 12,
    #[doc = "13: clk_wifi"]
    Hd = 13,
    #[doc = "14: clk_wifi"]
    He = 14,
    #[doc = "15: clk_wifi"]
    Hf = 15,
}
impl From<TestclkSel> for u8 {
    #[inline(always)]
    fn from(variant: TestclkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TestclkSel {
    type Ux = u8;
}
#[doc = "Field `TESTCLK_SEL` reader - Output clock selection for test"]
pub type TestclkSelR = crate::FieldReader<TestclkSel>;
impl TestclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestclkSel {
        match self.bits {
            0 => TestclkSel::H0,
            1 => TestclkSel::H1,
            2 => TestclkSel::H2,
            3 => TestclkSel::H3,
            4 => TestclkSel::H4,
            5 => TestclkSel::H5,
            6 => TestclkSel::H6,
            7 => TestclkSel::H7,
            8 => TestclkSel::H8,
            9 => TestclkSel::H9,
            10 => TestclkSel::Ha,
            11 => TestclkSel::Hb,
            12 => TestclkSel::Hc,
            13 => TestclkSel::Hd,
            14 => TestclkSel::He,
            15 => TestclkSel::Hf,
            _ => unreachable!(),
        }
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == TestclkSel::H0
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == TestclkSel::H1
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == TestclkSel::H2
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == TestclkSel::H3
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h4(&self) -> bool {
        *self == TestclkSel::H4
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h5(&self) -> bool {
        *self == TestclkSel::H5
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h6(&self) -> bool {
        *self == TestclkSel::H6
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h7(&self) -> bool {
        *self == TestclkSel::H7
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h8(&self) -> bool {
        *self == TestclkSel::H8
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_h9(&self) -> bool {
        *self == TestclkSel::H9
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_ha(&self) -> bool {
        *self == TestclkSel::Ha
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_hb(&self) -> bool {
        *self == TestclkSel::Hb
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_hc(&self) -> bool {
        *self == TestclkSel::Hc
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_hd(&self) -> bool {
        *self == TestclkSel::Hd
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_he(&self) -> bool {
        *self == TestclkSel::He
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn is_hf(&self) -> bool {
        *self == TestclkSel::Hf
    }
}
#[doc = "Field `TESTCLK_SEL` writer - Output clock selection for test"]
pub type TestclkSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, TestclkSel>;
impl<'a, REG> TestclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H0)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H1)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H2)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H3)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h4(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H4)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h5(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H5)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h6(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H6)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h7(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H7)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h8(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H8)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn h9(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::H9)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn ha(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::Ha)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn hb(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::Hb)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn hc(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::Hc)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn hd(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::Hd)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn he(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::He)
    }
    #[doc = "clk_wifi"]
    #[inline(always)]
    pub fn hf(self) -> &'a mut crate::W<REG> {
        self.variant(TestclkSel::Hf)
    }
}
#[doc = "A53/A72 warm reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarmrstnEn {
    #[doc = "0: enable A53/A72 warm reset"]
    B0 = 0,
    #[doc = "1: enable A53/A72 warm reset"]
    B1 = 1,
}
impl From<WarmrstnEn> for bool {
    #[inline(always)]
    fn from(variant: WarmrstnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WARMRSTN_EN` reader - A53/A72 warm reset enable"]
pub type WarmrstnEnR = crate::BitReader<WarmrstnEn>;
impl WarmrstnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WarmrstnEn {
        match self.bits {
            false => WarmrstnEn::B0,
            true => WarmrstnEn::B1,
        }
    }
    #[doc = "enable A53/A72 warm reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WarmrstnEn::B0
    }
    #[doc = "enable A53/A72 warm reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WarmrstnEn::B1
    }
}
#[doc = "Field `WARMRSTN_EN` writer - A53/A72 warm reset enable"]
pub type WarmrstnEnW<'a, REG> = crate::BitWriter<'a, REG, WarmrstnEn>;
impl<'a, REG> WarmrstnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable A53/A72 warm reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WarmrstnEn::B0)
    }
    #[doc = "enable A53/A72 warm reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WarmrstnEn::B1)
    }
}
#[doc = "A53/A72 DBGRSTN reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgrstnEn {
    #[doc = "0: enable A53/A72 DBGRSTN reset"]
    B0 = 0,
    #[doc = "1: enable A53/A72 DBGRSTN reset"]
    B1 = 1,
}
impl From<DbgrstnEn> for bool {
    #[inline(always)]
    fn from(variant: DbgrstnEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGRSTN_EN` reader - A53/A72 DBGRSTN reset enable"]
pub type DbgrstnEnR = crate::BitReader<DbgrstnEn>;
impl DbgrstnEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgrstnEn {
        match self.bits {
            false => DbgrstnEn::B0,
            true => DbgrstnEn::B1,
        }
    }
    #[doc = "enable A53/A72 DBGRSTN reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DbgrstnEn::B0
    }
    #[doc = "enable A53/A72 DBGRSTN reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DbgrstnEn::B1
    }
}
#[doc = "Field `DBGRSTN_EN` writer - A53/A72 DBGRSTN reset enable"]
pub type DbgrstnEnW<'a, REG> = crate::BitWriter<'a, REG, DbgrstnEn>;
impl<'a, REG> DbgrstnEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable A53/A72 DBGRSTN reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgrstnEn::B0)
    }
    #[doc = "enable A53/A72 DBGRSTN reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgrstnEn::B1)
    }
}
#[doc = "A53/A72 software reset wait for STANDBYWFI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoreSrstWfien {
    #[doc = "0: A53/A72 software reset is asserted after STANDBYWFI valid"]
    B0 = 0,
    #[doc = "1: A53/A72 software reset is asserted after STANDBYWFI valid"]
    B1 = 1,
}
impl From<CoreSrstWfien> for bool {
    #[inline(always)]
    fn from(variant: CoreSrstWfien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORE_SRST_WFIEN` reader - A53/A72 software reset wait for STANDBYWFI enable"]
pub type CoreSrstWfienR = crate::BitReader<CoreSrstWfien>;
impl CoreSrstWfienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CoreSrstWfien {
        match self.bits {
            false => CoreSrstWfien::B0,
            true => CoreSrstWfien::B1,
        }
    }
    #[doc = "A53/A72 software reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CoreSrstWfien::B0
    }
    #[doc = "A53/A72 software reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CoreSrstWfien::B1
    }
}
#[doc = "Field `CORE_SRST_WFIEN` writer - A53/A72 software reset wait for STANDBYWFI enable"]
pub type CoreSrstWfienW<'a, REG> = crate::BitWriter<'a, REG, CoreSrstWfien>;
impl<'a, REG> CoreSrstWfienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A53/A72 software reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CoreSrstWfien::B0)
    }
    #[doc = "A53/A72 software reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CoreSrstWfien::B1)
    }
}
#[doc = "A53/A72 warm reset wait for STANDBYWFI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoreWrstWifen {
    #[doc = "0: A53/A72 warm reset is asserted after STANDBYWFI valid"]
    B0 = 0,
    #[doc = "1: A53/A72 warm reset is asserted after STANDBYWFI valid"]
    B1 = 1,
}
impl From<CoreWrstWifen> for bool {
    #[inline(always)]
    fn from(variant: CoreWrstWifen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORE_WRST_WIFEN` reader - A53/A72 warm reset wait for STANDBYWFI enable"]
pub type CoreWrstWifenR = crate::BitReader<CoreWrstWifen>;
impl CoreWrstWifenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CoreWrstWifen {
        match self.bits {
            false => CoreWrstWifen::B0,
            true => CoreWrstWifen::B1,
        }
    }
    #[doc = "A53/A72 warm reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CoreWrstWifen::B0
    }
    #[doc = "A53/A72 warm reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CoreWrstWifen::B1
    }
}
#[doc = "Field `CORE_WRST_WIFEN` writer - A53/A72 warm reset wait for STANDBYWFI enable"]
pub type CoreWrstWifenW<'a, REG> = crate::BitWriter<'a, REG, CoreWrstWifen>;
impl<'a, REG> CoreWrstWifenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A53/A72 warm reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CoreWrstWifen::B0)
    }
    #[doc = "A53/A72 warm reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CoreWrstWifen::B1)
    }
}
#[doc = "A53/A72 dbg reset wait for STANDBYWFI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoreDbgrstWfien {
    #[doc = "0: A53 dgb reset is asserted after STANDBYWFI valid"]
    B0 = 0,
    #[doc = "1: A53 dgb reset is asserted after STANDBYWFI valid"]
    B1 = 1,
}
impl From<CoreDbgrstWfien> for bool {
    #[inline(always)]
    fn from(variant: CoreDbgrstWfien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORE_DBGRST_WFIEN` reader - A53/A72 dbg reset wait for STANDBYWFI enable"]
pub type CoreDbgrstWfienR = crate::BitReader<CoreDbgrstWfien>;
impl CoreDbgrstWfienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CoreDbgrstWfien {
        match self.bits {
            false => CoreDbgrstWfien::B0,
            true => CoreDbgrstWfien::B1,
        }
    }
    #[doc = "A53 dgb reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CoreDbgrstWfien::B0
    }
    #[doc = "A53 dgb reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CoreDbgrstWfien::B1
    }
}
#[doc = "Field `CORE_DBGRST_WFIEN` writer - A53/A72 dbg reset wait for STANDBYWFI enable"]
pub type CoreDbgrstWfienW<'a, REG> = crate::BitWriter<'a, REG, CoreDbgrstWfien>;
impl<'a, REG> CoreDbgrstWfienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A53 dgb reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CoreDbgrstWfien::B0)
    }
    #[doc = "A53 dgb reset is asserted after STANDBYWFI valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CoreDbgrstWfien::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Output clock selection for test"]
    #[inline(always)]
    pub fn testclk_sel(&self) -> TestclkSelR {
        TestclkSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - A53/A72 warm reset enable"]
    #[inline(always)]
    pub fn warmrstn_en(&self) -> WarmrstnEnR {
        WarmrstnEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A53/A72 DBGRSTN reset enable"]
    #[inline(always)]
    pub fn dbgrstn_en(&self) -> DbgrstnEnR {
        DbgrstnEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A53/A72 software reset wait for STANDBYWFI enable"]
    #[inline(always)]
    pub fn core_srst_wfien(&self) -> CoreSrstWfienR {
        CoreSrstWfienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A53/A72 warm reset wait for STANDBYWFI enable"]
    #[inline(always)]
    pub fn core_wrst_wifen(&self) -> CoreWrstWifenR {
        CoreWrstWifenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A53/A72 dbg reset wait for STANDBYWFI enable"]
    #[inline(always)]
    pub fn core_dbgrst_wfien(&self) -> CoreDbgrstWfienR {
        CoreDbgrstWfienR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Output clock selection for test"]
    #[inline(always)]
    #[must_use]
    pub fn testclk_sel(&mut self) -> TestclkSelW<CruMiscConSpec> {
        TestclkSelW::new(self, 0)
    }
    #[doc = "Bit 4 - A53/A72 warm reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn warmrstn_en(&mut self) -> WarmrstnEnW<CruMiscConSpec> {
        WarmrstnEnW::new(self, 4)
    }
    #[doc = "Bit 5 - A53/A72 DBGRSTN reset enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrstn_en(&mut self) -> DbgrstnEnW<CruMiscConSpec> {
        DbgrstnEnW::new(self, 5)
    }
    #[doc = "Bit 6 - A53/A72 software reset wait for STANDBYWFI enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_srst_wfien(&mut self) -> CoreSrstWfienW<CruMiscConSpec> {
        CoreSrstWfienW::new(self, 6)
    }
    #[doc = "Bit 7 - A53/A72 warm reset wait for STANDBYWFI enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_wrst_wifen(&mut self) -> CoreWrstWifenW<CruMiscConSpec> {
        CoreWrstWifenW::new(self, 7)
    }
    #[doc = "Bit 8 - A53/A72 dbg reset wait for STANDBYWFI enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_dbgrst_wfien(&mut self) -> CoreDbgrstWfienW<CruMiscConSpec> {
        CoreDbgrstWfienW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruMiscConSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Output clock selection for test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_misc_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_misc_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruMiscConSpec;
impl crate::RegisterSpec for CruMiscConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_misc_con::R`](R) reader structure"]
impl crate::Readable for CruMiscConSpec {}
#[doc = "`write(|w| ..)` method takes [`cru_misc_con::W`](W) writer structure"]
impl crate::Writable for CruMiscConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_MISC_CON to value 0"]
impl crate::Resettable for CruMiscConSpec {
    const RESET_VALUE: u32 = 0;
}
